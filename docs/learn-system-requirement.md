# LearnWiki 学习系统重构设计文档

> 版本：v1.0  
> 日期：2026-06-09  
> 状态：设计阶段

---

## 一、设计目标

将 LearnWiki 的学习模块从"课程平台式"的复杂系统，重构为**目标驱动、知识库原生**的轻量学习系统。

### 核心原则

1. **目标是开关** — 没有目标就没有学习动作，所有学习行为围绕目标展开
2. **知识库即素材库** — 学习内容全部来自 Wiki 知识点，不需要额外创建学习材料
3. **自动化优先** — 内容自动匹配目标、自动进入复习池、自动推荐考试时机
4. **简单直接** — 用户打开学习页就知道"现在该做什么"

---

## 二、系统总览

### 完整链路

```
设定目标
  → AI 联网搜索，生成由浅入深的推荐资源列表
    → 用户选择收录 → 内容进入知识库 → AI 编译成 Wiki 页面
      → 自动关联目标 → 加入学习列表（高亮提示）
        → 学习模式阅读（引导 + 渐进 + 即时检测）
          → 自动进入复习池（艾宾浩斯遗忘曲线）
            → 考试检测掌握度
              → 目标进度更新 / 达成
```

### 内容输入来源

| 来源 | 说明 |
|------|------|
| 剪贴板捕获 | 现有功能，不变 |
| 文件夹同步 | 新增功能，批量导入本地文件 |
| 目标推荐搜索 | 新增功能，AI 联网搜索推荐资源 |
| 手动添加 | 现有功能，不变 |

所有来源的内容进入系统后，统一走"目标匹配 → 学习 → 复习 → 考试"的循环。

---

## 三、目标系统

### 3.1 目标定义

目标是用户用一句话描述的学习意图，例如：
- "掌握 Rust 的所有权机制"
- "理解宏观经济运行逻辑"
- "学会 React Server Components"

### 3.2 目标生命周期

```
创建 → 活跃（学习中）→ 达成 / 归档
```

### 3.3 目标创建流程

1. 用户输入一句话描述目标
2. AI 分析目标，提取关键词和知识领域
3. AI 联网搜索相关学习资源
4. 生成**由浅入深的推荐列表**：
   - 🟢 入门（2-3 篇）：概念介绍、基本原理
   - 🟡 进阶（3-5 篇）：实现细节、对比分析
   - 🔴 深入（2-3 篇）：源码解读、最佳实践、边界情况
5. 同时扫描现有 Wiki 知识库，关联已有的相关页面
6. 展示：推荐资源列表 + 已有关联知识点 + 差距分析

### 3.4 推荐列表

每条推荐项包含：
- 标题
- 来源 URL
- 内容摘要
- 难度层级（入门/进阶/深入）
- 操作：收录 / 忽略

用户点"收录"→ 自动抓取内容 → 进入内容列表 → 编译成 Wiki 页面 → 关联目标

### 3.5 目标进度

- 进度 = 目标下所有关联知识点的综合掌握率
- 掌握率基于复习表现（ReviewSchedule 的 mastery 字段）
- 目标详情页展示：
  - 整体进度百分比
  - 知识点列表（按掌握度排序）
  - 薄弱知识点高亮

### 3.6 数据模型

```sql
CREATE TABLE goals (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  keywords TEXT,              -- AI 提取的关键词，JSON 数组
  status TEXT NOT NULL DEFAULT 'active',  -- active / achieved / archived
  progress REAL NOT NULL DEFAULT 0,       -- 0-100
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE goal_wiki_links (
  id TEXT PRIMARY KEY,
  goal_id TEXT NOT NULL REFERENCES goals(id),
  wiki_page_id TEXT NOT NULL,
  relevance_score REAL DEFAULT 0,
  source TEXT NOT NULL DEFAULT 'auto',   -- auto / manual
  is_new INTEGER NOT NULL DEFAULT 1,     -- 是否新加入（用于高亮）
  created_at TEXT NOT NULL,
  UNIQUE(goal_id, wiki_page_id)
);

CREATE TABLE goal_recommendations (
  id TEXT PRIMARY KEY,
  goal_id TEXT NOT NULL REFERENCES goals(id),
  title TEXT NOT NULL,
  url TEXT,
  summary TEXT,
  difficulty TEXT NOT NULL,     -- beginner / intermediate / advanced
  sort_order INTEGER NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending',  -- pending / imported / dismissed
  imported_content_id TEXT,    -- 收录后关联的内容 ID
  created_at TEXT NOT NULL
);
```

---

## 四、内容自动关联目标

### 4.1 触发时机

| 事件 | 触发匹配 |
|------|---------|
| 剪贴板内容保存 | 立即匹配 |
| 文件夹同步导入完成 | 批量匹配 |
| 手动添加内容 | 立即匹配 |
| Wiki 页面编译完成 | 立即匹配 |
| 新目标创建 | 反向扫描已有 Wiki 页面 |

### 4.2 匹配逻辑

- AI 对新内容/Wiki 页面与所有活跃目标进行语义匹配
- 匹配维度：关键词重叠、主题相关性、语义相似度
- 超过阈值 → 自动关联（标记 source = 'auto'）
- 可同时匹配多个目标

### 4.3 高亮提示

- 学习首页通知："有 N 个新知识点加入了你的目标「XXX」"
- 目标详情页中，新关联的知识点带 🆕 标签
- 用户查看后，标记 `is_new = 0`

### 4.4 手动关联

用户也可以在 Wiki 页面上手动将其关联到某个目标（标记 source = 'manual'）。

---

## 五、学习知识点

### 5.1 学习模式

Wiki 页面是学习的原子单位。在目标详情页点击某个知识点，进入学习模式：

**渐进式展示：**

1. **第一层：核心概念**（30秒能看完）
   - AI 从 Wiki 正文提取的关键定义/结论
   - 一句话总结"这个东西是什么"

2. **第二层：详细解释**
   - Wiki 正文完整展示
   - AI 生成的学习引导：核心是什么、跟什么相关、常见误区
   - 重点标注高亮

3. **第三层：延伸**
   - 前置知识提示（基于知识图谱）
   - 关联知识点推荐："学完这个，建议看看 XX"

### 5.2 即时检测

读完一个知识点后，立刻弹出 1-2 道轻量题目：
- 一道选择题或填空题
- 目的是强化记忆，不是打分
- 答完立刻显示对错 + 简短解释
- 完成后，该知识点正式进入复习池

### 5.3 学习状态

每个知识点在目标下有学习状态：

```
未学习 → 学习中（打开阅读）→ 已学习（完成即时检测）→ 复习中
```

---

## 六、日常复习

### 6.1 调度算法

复用现有的间隔重复机制（ReviewSchedule），基于艾宾浩斯遗忘曲线：
- ease_factor 动态调整
- interval_days 逐步增长
- 根据复习表现决定下次复习时间

### 6.2 复习范围

**只复习目标下的知识点。** 具体优先级：
1. 已过期（overdue）的知识点优先
2. 活跃目标下的知识点优先于已归档目标
3. 掌握度低的优先

### 6.3 复习格式（精简为 3 种）

| 格式 | 说明 | 适用场景 |
|------|------|---------|
| **选择题（Quiz）** | 4选1，AI 从 Wiki 内容生成题目和干扰项 | 辨析理解 |
| **填空（Cloze）** | Wiki 关键句挖空，用户填写 | 快速回忆概念 |
| **简述（Explain）** | 给出概念，要求用户用自己的话解释 | 深层掌握检测 |

### 6.4 复习交互

- 打开学习页 → 顶部显示"今日待复习 N 个"
- 一键开始 → 逐题作答 → 即时反馈
- 完成后更新 ReviewSchedule（下次复习时间、mastery）

### 6.5 数据模型

复用现有表，不需要改动：
- `review_schedules` — 调度记录
- `review_logs` — 复习日志

---

## 七、考试系统

### 7.1 定位

考试是对目标掌握度的**阶段性全面检测**，跟日常复习的区别：
- 复习 = 零散、每天几个、轻量
- 考试 = 集中、覆盖面广、有难度、有分数

### 7.2 触发方式

1. **系统推荐**：当某个目标下有较多知识点即将进入遗忘期时，提示"建议考一次"
2. **用户主动发起**：在目标详情页点击"开始考试"

### 7.3 出题范围

基于艾宾浩斯遗忘曲线圈定：
- 优先出即将过期和已过期的知识点
- 辅以少量已掌握的知识点（防止遗忘错觉）
- 按目标维度出题，一次考试针对一个目标

### 7.4 题目组成

| 题型 | 占比 | 说明 |
|------|------|------|
| 选择题 | ~50% | 4选1，有干扰项，需要思考 |
| 判断题 | ~20% | 对错判断 + 说明理由 |
| 简答/论述 | ~30% | 开放题，AI 评分 |

**题量：** 20-30 题/次（根据目标下知识点数量动态调整）

### 7.5 难度设计

- 不是直接从 Wiki 原文出题，而是**变换角度**：
  - 跨知识点关联题
  - 反向提问（给结论问原因）
  - 应用场景题（给场景问用什么）
  - 易混淆概念辨析
- 动态难度：知识点 mastery 越高，出题角度越刁钻

### 7.6 考后反馈

1. **总分 + 等级**（A/B/C/D）
2. **逐题解析**：正确答案 + 解释
3. **薄弱诊断**：
   - 哪些知识点答错了
   - 错误类型分析（记忆模糊/理解偏差/完全不会）
   - 这些知识点自动加入高频复习
4. **趋势对比**：跟上次该目标的考试结果对比（进步/退步）
5. **建议**：哪些知识点需要回去重新学习

### 7.7 数据模型

```sql
CREATE TABLE exams (
  id TEXT PRIMARY KEY,
  goal_id TEXT NOT NULL REFERENCES goals(id),
  title TEXT,
  total_questions INTEGER NOT NULL,
  score REAL,                    -- 0-100
  grade TEXT,                    -- A/B/C/D
  status TEXT NOT NULL DEFAULT 'in_progress',  -- in_progress / completed
  started_at TEXT NOT NULL,
  completed_at TEXT,
  diagnosis_json TEXT,           -- 薄弱诊断结果 JSON
  created_at TEXT NOT NULL
);

CREATE TABLE exam_questions (
  id TEXT PRIMARY KEY,
  exam_id TEXT NOT NULL REFERENCES exams(id),
  wiki_page_id TEXT NOT NULL,
  question_type TEXT NOT NULL,   -- choice / judgment / essay
  question_json TEXT NOT NULL,   -- 题目内容 JSON
  user_answer TEXT,
  correct_answer TEXT,
  is_correct INTEGER,
  score REAL,                    -- 该题得分
  ai_feedback TEXT,              -- AI 评分反馈（论述题）
  sort_order INTEGER NOT NULL,
  answered_at TEXT
);
```

---

## 八、文件夹同步功能

### 8.1 功能概述

在内容模块新增"同步文件夹"功能，支持从本地文件夹批量导入文件到内容列表。

### 8.2 设置项

在设置页新增"同步文件夹"配置区：
- 支持添加多个文件夹路径（系统文件夹选择器）
- 每个路径可独立启用/禁用
- 显示上次同步时间
- 支持删除已配置的路径

### 8.3 支持格式

| 类型 | 格式 |
|------|------|
| 文档 | `.md` / `.txt` / `.pdf` / `.docx` |
| 电子书 | `.epub` |
| 图片 | `.png` / `.jpg` / `.jpeg` / `.webp` |

### 8.4 同步逻辑

**触发方式：** 用户在内容页点击"同步"按钮（手动触发）

**扫描规则：**
- 递归遍历所有子目录
- 按支持格式过滤文件
- 去重判断：
  - 文件路径相同 + mtime 未变 → 跳过
  - 文件路径相同 + mtime 更新 → 重新导入（标记"已更新"）
  - 新文件 → 导入

**导入处理：**
- 文档类（md/txt/docx/pdf/epub）→ 提取文本内容，存为文本类型内容条目
- 图片类（png/jpg/webp）→ 复制到应用存储，存为图片类型内容条目
- 来源标记：记录原始文件路径 + 所属同步文件夹

### 8.5 同步结果展示

同步完成后弹出结果面板：
- ✅ 新导入 X 个文件（列出文件名）
- 🔄 更新 Y 个文件（列出文件名）
- ⏭️ 跳过 Z 个（未变化）
- 每个文件可点击查看内容

### 8.6 内容列表中的展示

- 同步进来的内容带 📁 标识和"来自：文件夹名"标签
- 顶部横幅高亮提示："有 N 条新内容待编译为知识点" + "批量编译"按钮
- 单条也可点击"编译为 Wiki"

### 8.7 与目标的联动

文件同步导入 → 编译成 Wiki 页面 → 触发目标匹配 → 命中则自动关联目标 → 加入学习列表

### 8.8 数据模型

```sql
CREATE TABLE sync_folders (
  id TEXT PRIMARY KEY,
  path TEXT NOT NULL UNIQUE,
  enabled INTEGER NOT NULL DEFAULT 1,
  last_synced_at TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE sync_records (
  id TEXT PRIMARY KEY,
  folder_id TEXT NOT NULL REFERENCES sync_folders(id),
  file_path TEXT NOT NULL,
  file_name TEXT NOT NULL,
  file_size INTEGER,
  file_mtime TEXT NOT NULL,
  file_type TEXT NOT NULL,       -- md / txt / pdf / docx / epub / image
  content_id TEXT,               -- 关联的内容条目 ID
  status TEXT NOT NULL DEFAULT 'imported',  -- imported / updated / error
  synced_at TEXT NOT NULL,
  UNIQUE(folder_id, file_path)
);
```

### 8.9 Tauri Commands

```rust
// 设置相关
#[tauri::command]
async fn add_sync_folder(path: String) -> Result<SyncFolder, String>;

#[tauri::command]
async fn remove_sync_folder(id: String) -> Result<(), String>;

#[tauri::command]
async fn get_sync_folders() -> Result<Vec<SyncFolder>, String>;

#[tauri::command]
async fn update_sync_folder(id: String, enabled: bool) -> Result<SyncFolder, String>;

// 同步操作
#[tauri::command]
async fn start_sync(folder_id: Option<String>) -> Result<SyncResult, String>;

#[tauri::command]
async fn get_sync_history(folder_id: String, limit: u32) -> Result<Vec<SyncRecord>, String>;
```

---

## 九、现有功能改造映射

### 9.1 复用（不改动）

| 现有组件/模块 | 在新系统中的角色 |
|-------------|----------------|
| ReviewSchedule + 间隔重复算法 | 日常复习调度 + 考试出题范围依据 |
| Wiki 页面体系 | 学习的原子单位 |
| 知识图谱 | 前置知识推荐、知识点串联 |
| 内容捕获 → Wiki 编译 | 不变，加上自动目标匹配 |
| review_schedules / review_logs 表 | 不变 |

### 9.2 改造

| 现有组件 | 改造方向 |
|---------|---------|
| LearningPath | → Goal（目标），去掉 Module 层级 |
| LearningDashboard | → 新首页：今日复习 / 目标列表 / 考试建议 |
| TaskRecommendations | → 目标推荐列表（联网搜索资源） |
| Discovery / ReadingInbox | 加上"目标驱动搜索"触发源 |
| ComprehensiveQuiz | → 考试系统 |
| KnowledgeHealth | → 目标进度面板（简化） |
| Quiz 组件 | 保留，用于日常复习 + 考试选择题 |
| Cloze 组件 | 保留，用于日常复习 |
| Explain 组件 | 保留，用于日常复习 + 考试论述题 |

### 9.3 删除

| 删除项 | 理由 |
|--------|------|
| Module（模块）| 目标直接关联 Wiki 页面，不需要中间层 |
| PracticeTask / TaskBoard | 不再需要手动任务管理 |
| Rapid Fire | 格式冗余 |
| Ordering | 格式冗余 |
| Error Hunt | 格式冗余 |
| Leaderboard（排行榜）| 个人工具不需要竞争 |
| OnboardingWizard | 新流程简化，设定目标本身就是入口 |
| TaskSolutions / TaskDetail | 跟任务系统一起去掉 |
| PracticeSandbox / AdvancedSandbox | 过重 |

---

## 十、页面结构

### 10.1 学习模块页面

```
学习（LearningView）
│
├── 首页（LearningHome）
│   ├── 今日复习卡片："待复习 N 个" → 一键开始
│   ├── 我的目标列表（进度条 + 状态）
│   ├── 新知识点通知："有 N 个新知识点加入目标 XXX"
│   └── 考试建议："目标 XXX 建议测一次"
│
├── 目标详情页（GoalDetail）
│   ├── 目标描述 + 整体进度（圆环/进度条）
│   ├── 推荐资源列表（由浅入深，可收录/忽略）
│   ├── 已关联知识点列表（掌握度 + 学习状态 + 新标记）
│   ├── 差距分析："还需要了解 XX 方面"
│   └── 操作：开始学习 / 发起考试
│
├── 学习模式（LearnMode）
│   ├── 渐进式内容展示
│   ├── AI 学习引导
│   └── 即时检测（1-2 题）
│
├── 复习会话（ReviewSession）
│   ├── Quiz（选择题）
│   ├── Cloze（填空）
│   └── Explain（简述）
│
├── 考试（ExamView）
│   ├── 考试进行中（混合题型）
│   └── 考后报告（ExamReport）
│       ├── 分数 + 等级
│       ├── 逐题解析
│       ├── 薄弱诊断
│       └── 趋势对比
│
└── 创建目标（GoalCreate）
    ├── 输入目标描述
    ├── AI 搜索中...
    └── 推荐资源列表预览 → 确认创建
```

### 10.2 设置页新增

```
设置 → 同步文件夹
├── 已配置文件夹列表（路径 / 启用状态 / 上次同步时间）
├── 添加文件夹按钮
└── 每个文件夹的操作：启用/禁用 / 删除
```

### 10.3 内容列表页改动

```
内容列表
├── 顶部新增"同步"按钮
├── 同步后横幅："N 条新内容待编译" + "批量编译"按钮
├── 列表项增加来源标签（📁 来自文件夹 / 📋 来自剪贴板）
└── 同步结果弹窗
```

---

## 十一、Tauri Commands 汇总

### 目标相关

```rust
#[tauri::command]
async fn create_goal(title: String, description: Option<String>) -> Result<Goal, String>;

#[tauri::command]
async fn get_goals(status: Option<String>) -> Result<Vec<Goal>, String>;

#[tauri::command]
async fn get_goal(id: String) -> Result<GoalDetail, String>;

#[tauri::command]
async fn update_goal(id: String, title: Option<String>, description: Option<String>, status: Option<String>) -> Result<Goal, String>;

#[tauri::command]
async fn delete_goal(id: String) -> Result<(), String>;

// 目标推荐
#[tauri::command]
async fn search_goal_resources(goal_id: String) -> Result<Vec<GoalRecommendation>, String>;

#[tauri::command]
async fn import_recommendation(recommendation_id: String) -> Result<String, String>;

#[tauri::command]
async fn dismiss_recommendation(recommendation_id: String) -> Result<(), String>;

// 目标关联
#[tauri::command]
async fn link_wiki_to_goal(goal_id: String, wiki_page_id: String) -> Result<(), String>;

#[tauri::command]
async fn unlink_wiki_from_goal(goal_id: String, wiki_page_id: String) -> Result<(), String>;

#[tauri::command]
async fn get_goal_wiki_pages(goal_id: String) -> Result<Vec<GoalWikiItem>, String>;

// 自动匹配
#[tauri::command]
async fn match_content_to_goals(content_id: String) -> Result<Vec<MatchResult>, String>;

#[tauri::command]
async fn match_wiki_to_goals(wiki_page_id: String) -> Result<Vec<MatchResult>, String>;
```

### 考试相关

```rust
#[tauri::command]
async fn create_exam(goal_id: String) -> Result<Exam, String>;

#[tauri::command]
async fn get_exam(id: String) -> Result<ExamDetail, String>;

#[tauri::command]
async fn submit_exam_answer(question_id: String, answer: String) -> Result<AnswerResult, String>;

#[tauri::command]
async fn complete_exam(exam_id: String) -> Result<ExamReport, String>;

#[tauri::command]
async fn get_exam_history(goal_id: String) -> Result<Vec<ExamSummary>, String>;
```

### 学习模式相关

```rust
#[tauri::command]
async fn get_learning_content(wiki_page_id: String) -> Result<LearningContent, String>;

#[tauri::command]
async fn mark_as_learned(goal_id: String, wiki_page_id: String) -> Result<(), String>;

#[tauri::command]
async fn generate_instant_quiz(wiki_page_id: String) -> Result<Vec<QuizQuestion>, String>;
```

### 文件夹同步相关

```rust
#[tauri::command]
async fn add_sync_folder(path: String) -> Result<SyncFolder, String>;

#[tauri::command]
async fn remove_sync_folder(id: String) -> Result<(), String>;

#[tauri::command]
async fn get_sync_folders() -> Result<Vec<SyncFolder>, String>;

#[tauri::command]
async fn update_sync_folder(id: String, enabled: bool) -> Result<SyncFolder, String>;

#[tauri::command]
async fn start_sync(folder_id: Option<String>) -> Result<SyncResult, String>;

#[tauri::command]
async fn get_sync_history(folder_id: String, limit: u32) -> Result<Vec<SyncRecord>, String>;
```

---

## 十二、实施建议

### 分阶段开发

**Phase 1：目标 + 日常复习简化（核心循环）**
- 实现 Goal CRUD
- Wiki 页面关联目标（手动）
- 复习系统精简为 3 种格式
- 学习首页重构
- 预计工期：1-2 周

**Phase 2：考试系统**
- 考试出题逻辑（基于遗忘曲线）
- 混合题型（选择/判断/论述）
- 考后报告 + 薄弱诊断
- 预计工期：1-2 周

**Phase 3：目标推荐搜索 + 自动关联**
- 联网搜索 + 推荐列表生成
- 内容自动匹配目标
- 学习模式（渐进展示 + 即时检测）
- 预计工期：1-2 周

**Phase 4：文件夹同步**
- 设置页配置
- 同步逻辑实现
- 内容列表展示改造
- 预计工期：1 周

**Phase 5：清理旧代码**
- 删除 Module / PracticeTask / TaskBoard 等废弃组件
- 数据库迁移（旧表处理）
- 预计工期：2-3 天

---

## 十三、附录

### A. 删除的文件清单

```
src/features/learning/OrderingReview.tsx
src/features/learning/RapidFireSession.tsx
src/features/learning/ErrorHuntReview.tsx
src/features/learning/TaskBoard.tsx
src/features/learning/TaskCard.tsx
src/features/learning/TaskDetail.tsx
src/features/learning/TaskSolutions.tsx
src/features/learning/TaskRecommendations.tsx
src/features/learning/PracticeSandbox.tsx
src/features/learning/AdvancedSandbox.tsx
src/features/learning/KnowledgeLinking.tsx
src/features/learning/AdaptiveRecommendations.tsx
src/features/learning/OnboardingWizard.tsx
src/features/learning/EmptyState.tsx
src/features/learning/EmptyTasks.tsx
src/features/ranking/Leaderboard.tsx
```

### B. 保留并改造的文件

```
src/features/learning/LearningView.tsx          → 重构 tab 结构
src/features/learning/LearningDashboard.tsx     → 重构为新首页
src/features/learning/ReviewSession.tsx         → 精简为 3 种格式
src/features/learning/ReviewList.tsx            → 保留
src/features/learning/ComprehensiveQuiz.tsx     → 改造为考试组件
src/features/learning/ClozeReview.tsx           → 保留
src/features/learning/ExplainReview.tsx         → 保留
src/features/learning/KnowledgeHealth.tsx       → 简化为目标进度
src/features/learning/review/StandardReview.tsx → 保留
src/features/learning/review/ReviewSummary.tsx  → 保留
src/stores/learningStore.ts                     → 大幅重构
src/services/learningService.ts                 → 大幅重构
src/types/learning.ts                           → 重写类型定义
```
