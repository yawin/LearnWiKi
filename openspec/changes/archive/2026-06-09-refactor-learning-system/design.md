## Context

当前 LearnWiki 学习模块基于 `learning_paths → modules → practice_tasks` 三层架构（Migration 017），代码分布在 `learning.rs`（3301行）、`learningStore.ts`（538行）、`learningService.ts`（637行）、`learning.ts` 类型（426行）以及 30 个前端组件中。

数据库迁移 027-030 已部署（goals / exams / goal_recommendations / sync 表），后端部分 Goal CRUD 命令已实现（`create_goal`、`link_wiki_to_goal` 等），但前端仍使用旧的 LearningPath 体系。新旧系统在同一个 Tab 下并存，用户面对两套入口。

## Goals / Non-Goals

**Goals:**
- 用 Goal（目标）替代 LearningPath + Module 作为用户学习行为的组织单元
- 将复习范围从"所有 Wiki 页面"限定为"目标下的知识点"
- 简化复习格式（6 → 3），保留 Quiz / Cloze / Explain
- 新增考试系统和文件夹同步
- 拆分 `learning.rs`，解决单文件过大问题
- 设计 AI 降级方案，确保核心流程在 AI 不可用时仍然可用

**Non-Goals:**
- 不修改 Wiki 编译引擎核心逻辑（仅增加目标关联触发点）
- 不修改内容捕获核心逻辑（仅增加自动匹配触发点）
- 不修改知识图谱核心逻辑（仅增加前置知识推荐查询）
- 不引入新的外部 AI 服务（复用现有 DeepSeek API）

## Decisions

### 1. 后端文件拆分：按领域模块化

```
src-tauri/src/commands/
  goal.rs        ← 目标 CRUD + 关联 + 推荐搜索
  exam.rs        ← 考试创建 + 评卷 + 报告生成
  review.rs      ← 复习会话 + 3 种格式
  sync.rs        ← 文件夹同步
  learning.rs    ← 保留旧系统命令（Phase 5 删除）
```

**理由**: `learning.rs` 已经 3301 行，再添加新功能将无法维护。按领域拆分后每个文件控制在 500-800 行。`learning.rs` 在 Phase 5 整体移除。

**替代方案**: 不拆分，继续在 `learning.rs` 添加代码 → 拒绝。3301 行已超过可维护阈值。

### 2. AI 降级方案：三级回退

```
一级（理想路径）  AI 语义匹配 / AI 搜索推荐 / AI 题目生成
        │
        ├─ AI 不可用
        ▼
二级（规则降级）  关键词重叠匹配 / 预设题库模板 / 手动关联
        │
        ├─ 完全离线
        ▼
三级（纯手动）    用户手动关联 Wiki → 目标 / 手动录入学习资源
```

**触发条件**: LLM 调用超时（>30s）或连续 3 次失败 → 自动降级。用户可手动切换回 AI 模式。

**理由**: 需求文档的全链路都依赖 AI，但 DeepSeek API 可能不稳定。核心操作（创建目标、关联 Wiki、开始复习）不能因 AI 不可用而阻塞。

**降级 UX**: 用非阻断 toast 提示"AI 暂不可用，使用本地匹配结果"，不弹模态框。

### 3. 旧系统清理前置

需求文档将清理放在 Phase 5（最后），但新旧系统并行会让用户在 4 个 Phase 期间面对两套入口。调整方案：

- **Phase 1 同步执行**: 隐藏旧系统入口（LearningPath / Module / TaskBoard 的 UI 入口在 LearningView 中移除），但保留后端命令和数据
- **Phase 5 彻底清理**: 删除后端命令、数据库表、前端组件

**理由**: 用户从 Phase 1 开始就只看到新的目标系统，避免困惑。旧数据保留在数据库中，必要时可通过 CLI 查询。

### 4. 考试中断恢复

`exams` 表增加状态处理：

```
in_progress → 用户关闭 → 下次打开检测未完成考试 → 弹窗"上次考试未完成，继续还是重新开始？"
            → 用户选择继续 → 恢复答题进度（已答题目保留）
            → 用户选择重新开始 → 清空已有答案，重新生成题目
```

实现方式: 在 `exam_questions` 表中记录 `answered_at` 字段 — 已填充的题目跳过，未填充的继续答题。

### 5. 复习格式数据迁移

迁移策略：

```
review_logs 中 format IN ('rapid_fire', 'ordering', 'error_hunt') 的记录
  → 不删除数据
  → 在 review_logs 表新增 migration_note 字段标记"已废弃"
  → 前端只查询 format IN ('quiz', 'cloze', 'explain')
```

**理由**: 删除用户学习数据不可逆。保留历史记录供 debug，但前端不再展示。

### 6. 自动匹配阈值

| 匹配方式 | 阈值 | 说明 |
|----------|------|------|
| 关键词重叠率 | ≥ 30% | 目标 keywords 与 Wiki 页面标题/tags 的重叠比例 |
| 语义相似度 | ≥ 0.6 | cosine similarity（AI 匹配路径） |
| 手动关联 | 无阈值 | 用户显式操作 |

每个内容/Wiki 页面最多自动关联 **3 个目标**（按得分排序取 top-3），防止 1 个页面挂到 10 个目标上。

### 7. 文件夹同步边界处理

| 场景 | 处理 |
|------|------|
| 两个同步文件夹路径重叠 | 配置时检测路径包含关系，拒绝添加子目录 |
| 文件在外部被删除 | sync 时检测文件不存在 → status = 'missing'，内容条目保留 |
| 超大文件（>50MB） | 跳过并记录 error，不阻塞其他文件 |
| 非文本 .txt（二进制） | 读取前 4KB 检测 null byte → 标记为 binary，不导入 |

## Risks / Trade-offs

| 风险 | 影响 | 缓解 |
|------|------|------|
| AI 生成的考试题目质量不稳定 | 考试公允性下降 | 提供"重新出题"按钮；论述题 AI 评分仅供参考，用户可手动覆盖 |
| 旧系统数据丢失（LearningPath / Module） | 用户学习历史不可追溯 | Phase 1 隐藏入口但保留数据；Phase 5 删除前做一次数据库快照 |
| 目标系统过于简化 | 高级用户的深度需求不满足 | Goal 保留 description 字段可存储结构化学习计划；后续可扩展子目标 |
| 复习仅限目标下知识点 | 非目标 Wiki 页面无复习提醒 | 在 Wiki 浏览页增加"加入目标"快捷入口，鼓励用户主动关联 |
| `learning.rs` 拆分后函数引用变更 | Rust 编译错误 / 前端 import 断裂 | 拆分时保持 Tauri command 注册名称不变，前端无感知 |

## Migration Plan

1. **Phase 1**: 部署新 Goal 系统 + 隐藏旧 UI（回滚：恢复旧 UI 入口，2 分钟）
2. **Phase 2**: 考试系统上线（回滚：隐藏考试入口，1 分钟）
3. **Phase 3**: AI 搜索 + 自动匹配（回滚：关闭自动匹配 flag，1 分钟）
4. **Phase 4**: 文件夹同步（回滚：隐藏同步按钮，1 分钟）
5. **Phase 5**: 删除旧代码 + 数据迁移（回滚：从 Git 恢复，但数据库迁移不可逆 → 需提前备份）

Phase 5 执行前务必运行 `pg_dump` 或 `sqlite3 .backup` 备份数据库。

## Open Questions

1. **AI 搜索用什么搜索引擎？** 需求文档提到"AI 联网搜索"但未指定服务（Google Custom Search / Bing / SerpAPI）。需要确认 API Key 和费用。
2. **考试论述题 AI 评分标准？** "用自己的话解释"的评分需要明确 rubric（关键词覆盖 / 语义理解 / 逻辑完整性）。
3. **sync_folders 数据迁移路径确认**：需求文档未明确是否需要 Tauri command 注册的路径配置 → 建议用 `get_sync_folders` 初始化时自动回填历史。
