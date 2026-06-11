## Why

当前学习模块基于"Module → Task"课程平台式架构（Migration 017），存在概念层级过多、内容与学习脱节、系统冗杂三个根本问题。用户需要的是**以目标驱动、知识库原生**的轻量学习体验。此次重构将学习模块从"课程平台"转型为"个人学习助手"。

## What Changes

- 引入 Goal（目标）作为学习行为的最小驱动力，替代 LearningPath + Module 双层体系
- 学习内容全部来自 Wiki 知识点，不再单独创建学习材料
- 复习格式从 6 种精简为 3 种（Quiz / Cloze / Explain）—— **BREAKING**: RapidFire、Ordering、ErrorHunt 删除，已有数据需迁移
- 新增考试系统：基于遗忘曲线的阶段检测，支持选择/判断/论述混合题型
- 新增文件夹同步：本地文件批量导入内容列表
- 新增 AI 联网搜索目标推荐资源 + 自动匹配
- **BREAKING**: 删除 LearningPath、Module、PracticeTask、TaskBoard、排行榜等旧组件
- **BREAKING**: learning.rs（3301行）拆分为 goal.rs / exam.rs / review.rs / sync.rs

## Capabilities

### New Capabilities
- `goal-management`: 目标的创建、生命周期管理、进度追踪、与 Wiki 知识点的关联
- `exam-system`: 基于目标的阶段性考试，混合题型出卷、考后诊断报告、趋势对比
- `goal-recommendations`: AI 联网搜索学习资源，生成由浅入深的推荐列表
- `content-goal-matching`: 内容/Wiki 页面与目标的自动语义匹配
- `learning-mode`: 渐进式知识点阅读 + 即时检测
- `folder-sync`: 本地文件夹配置、文件扫描、去重导入、与目标和 Wiki 联动

### Modified Capabilities
- `review-system`: 复习格式从 6 种简化为 3 种（Quiz / Cloze / Explain），复习范围限定为目标下的知识点，非目标知识点不再进入复习池
- `learning-dashboard`: 首页从"课程地图"重构为"今日复习 + 目标列表 + 考试建议"

## Impact

| 层级 | 影响 |
|------|------|
| 数据库 | 新增 goals / goal_wiki_links / exams / exam_questions / goal_recommendations / sync_folders / sync_records（迁移 027-030 已存在）；Phase 5 移除 learning_paths / modules / practice_tasks / task_daily_logs |
| Rust 后端 | learning.rs（3301行）拆分为 goal.rs / exam.rs / review.rs / sync.rs；新增 ~20 个 Tauri command |
| TypeScript 类型 | `src/types/learning.ts`（426行）重写，移除 Module / PracticeTask / LearningPath 类型，保留并扩展 Goal / Exam 类型 |
| 状态管理 | `learningStore.ts`（538行）大幅重构，移除旧 action，新增 goal/exam/sync store |
| 前端组件 | 删除 16 个旧组件（OnboardingWizard / TaskBoard / TaskDetail / PracticeSandbox 等）；保留并改造 13 个组件 |
| AI 依赖 | 全链路新增 AI 调用点：目标搜索 → 内容编译 → 语义匹配 → 题目生成 → 论述评分。需要设计降级方案（关键词匹配 fallback / 缓存 / grace period） |
