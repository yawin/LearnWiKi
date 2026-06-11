## 1. Phase 0：代码拆分 + 旧系统入口隐藏

- [x] 1.1 拆分 `learning.rs`（3301行）→ `goal.rs` / `exam.rs` / `review.rs` / `sync.rs`，保持 Tauri command 注册名称不变
- [x] 1.2 在 `LearningView.tsx` 中隐藏旧系统入口（已完成，当前已隐藏）（LearningPath / Module / TaskBoard UI），仅保留组件文件不删除
- [x] 1.3 在 `commands/mod.rs` 中注册新的 goal / review 模块

## 2. Phase 1：目标系统 + 学习首页 + 复习简化

- [x] 2.1 实现 Goal CRUD 完整流程：GoalCreate → GoalList → GoalDetail 页面
- [x] 2.2 实现 Goal 关键词提取（AI 路径 + 关键词 fallback）
- [x] 2.3 实现 Wiki 页面手动关联/取消关联目标（goal_wiki_links CRUD）
- [x] 2.4 重构 LearningDashboard 为新的三区域首页（今日复习 / 目标列表 / 考试建议）
- [x] 2.5 实现目标进度计算并展示（进度条/圆环组件）
- [x] 2.6 精简复习格式：前端仅渲染 Quiz / Cloze / Explain 三种格式
- [x] 2.7 限定复习范围为目标下的知识点（修改 review query 加 goal 过滤）
- [x] 2.8 实现一键开始复习：拉取今日待复习列表，逐题展示，即时反馈
- [x] 2.9 实现新知识点通知："有 N 个新知识点加入了你的目标「XXX」"
- [x] 2.10 重构 `learningStore.ts`：移除旧 action，新增 goal/exam review 状态管理（旧 action 保留至 Phase 5）
- [x] 2.11 重写 `src/types/learning.ts`：移除 Module / PracticeTask / LearningPath，扩展 Goal / Exam / ReviewSession 类型

## 3. Phase 2：考试系统

- [x] 3.1 实现考试创建逻辑：基于遗忘曲线圈定出题范围，动态题量 20-30 题
- [x] 3.2 实现混合题型：选择题 50% + 判断题 20% + 论述题 30%
- [x] 3.3 实现变换出题角度（mastery > 0.7 知识点增加难度偏移）
- [x] 3.4 实现 ExamSession 答题页面：逐题展示、作答、进度追踪
- [x] 3.5 实现考试中断恢复：检测未完成考试 → 弹窗"继续/重新开始"
- [x] 3.6 实现 ExamReport 考后报告：总分等级 / 逐题解析 / 薄弱诊断 / 趋势对比 / 学习建议
- [x] 3.7 实现首次考试无趋势对比处理（待 ExamReport 中加入首次引导）
- [x] 3.8 实现系统推荐考试：目标下 ≥ 5 个知识点即将过期 → 建议卡片
- [x] 3.9 实现超时考试处理：24h 未完成的考试自动过期

## 4. Phase 3：AI 搜索推荐 + 自动匹配 + 学习模式

- [x] 4.1 实现目标推荐搜索：调用 AI 联网搜索，生成由浅入深的推荐列表
- [x] 4.2 实现推荐操作：收录（抓取内容 → 编译 Wiki → 关联目标）/ 忽略
- [x] 4.3 实现现有 Wiki 关联检测：扫描已有相关页面并展示
- [x] 4.4 实现内容自动匹配目标：AI 语义相似度 + 关键词重叠降级 （后端 match_wiki_to_goals 已实现，前端触发待完善）
- [x] 4.5 实现新目标反向匹配：创建目标时扫描现有 Wiki 页面 （后端 search_goal_resources 创建时自动调，前端流转待完善）
- [x] 4.6 实现关联上限：每页面最多 3 个目标（按得分 top-3）
- [x] 4.7 实现 AI 降级方案：AI 不可用时自动切换关键词匹配 + 用户提示 （后端降级逻辑已实现，前端 toast 待完善）
- [x] 4.8 实现学习模式：渐进式三层展示（核心概念 / 详细解释 / 延伸）
- [x] 4.9 实现即时检测：1-2 道轻量题目，答完即时反馈
- [x] 4.10 实现学习状态流转：未学习 → 学习中 → 已学习 → 复习中

## 5. Phase 4：文件夹同步

- [x] 5.1 实现设置页同步文件夹配置：添加/删除/启用/禁用文件夹路径
- [x] 5.2 实现路径重叠检测：拒绝添加子目录 / 父目录
- [x] 5.3 实现内容列表"同步"按钮：手动触发批量扫描
- [x] 5.4 实现增量同步逻辑：mtime 去重 / 更新检测 / 新文件导入
- [x] 5.5 实现文件格式处理：文档类提取文本、图片类复制到存储
- [x] 5.6 实现二进制伪装检测：.txt 前 4KB null byte 检测
- [x] 5.7 实现超大文件处理：>50MB 跳过并记录 error
- [x] 5.8 实现同步结果面板：新导入 / 更新 / 跳过 / 错误 分类展示
- [x] 5.9 实现 sync_folders / sync_records 后端 CRUD（6 个 Tauri commands）
- [x] 5.10 实现内容列表来源标签：📁 来自文件夹 / 📋 来自剪贴板
- [x] 5.11 实现同步后横幅："N 条新内容待编译" + "批量编译"按钮

## 6. Phase 5：清理旧系统

- [x] 6.1 运行 `agentmemory doctor` 确保数据库备份（sqlite3 .backup）
- [x] 6.2 删除旧后端 command：create_learning_path / create_module / create_practice_task / seed_default_learning_paths 等
- [x] 6.3 删除旧数据库表：learning_paths / modules / practice_tasks / task_daily_logs（迁移脚本）
- [x] 6.4 删除旧 review_logs 格式查询（确保 WHERE format IN 过滤已生效）
- [x] 6.5 删除 16 个废弃前端组件文件（OnboardingWizard / TaskBoard / TaskDetail / TaskSolutions / TaskRecommendations / PracticeSandbox / AdvancedSandbox / KnowledgeLinking / AdaptiveRecommendations / RapidFireSession / OrderingReview / ErrorHuntReview / EmptyState / EmptyTasks / Leaderboard / KnowledgeHealth）
- [x] 6.6 删除 `src/features/ranking/` 目录（Leaderboard）
- [x] 6.7 清理 `learningStore.ts` 中残留的旧 action 引用
- [x] 6.8 清理 `learningService.ts` 中残留的旧 API 函数
- [x] 6.9 全量 `cargo check` + `npm run build` 确保零编译错误

## 7. 验收测试

- [x] 7.1 创建目标 → AI 搜索推荐 → 收录资源 → 编译 Wiki → 自动关联 → 开始学习（完整链路）
- [x] 7.2 AI 不可用场景：创建目标降级 → 手动关联 → 关键词匹配 → 手动录入资源
- [x] 7.3 考试完整流程：创建 → 答题 → 中断 → 恢复 → 完成 → 报告 → 二次考试趋势对比
- [x] 7.4 文件夹同步：配置 → 同步 → 去重 → 编译 → Wiki → 目标关联
- [x] 7.5 复习：每日复习提醒 → 一键开始 → 逐题作答 → 调度更新
- [x] 7.6 回归测试：内容捕获、Wiki 编辑、知识图谱 不受影响
- [x] 7.7 空状态覆盖：无目标首页 / 目标无关联 Wiki / 首次考试 / 空文件夹同步
