## ADDED Requirements

### Requirement: 创建目标
系统 SHALL 允许用户通过一句话描述创建学习目标。系统 MUST 在创建时自动提取关键词（AI 路径）或使用用户手动输入的关键词（降级路径），关键词存储为 JSON 数组。

#### Scenario: 正常创建目标
- **WHEN** 用户输入目标描述"掌握 Rust 的所有权机制"并确认
- **THEN** 系统创建 Goal 记录，status 为 "active"，progress 为 0，keywords 由 AI 提取并填充

#### Scenario: AI 不可用时的降级创建
- **WHEN** AI 关键词提取失败或超时
- **THEN** 系统仍创建 Goal，keywords 为空数组 "[]"，提示用户可手动添加关键词

### Requirement: 目标生命周期管理
每个 Goal SHALL 有明确的生命周期状态：active（活跃）、achieved（达成）、archived（归档）。状态转换规则：active → achieved 由用户手动标记；任何状态 → archived 由用户手动操作；archived → active 允许恢复。

#### Scenario: 标记目标达成
- **WHEN** 用户在目标详情页点击"标记达成"
- **THEN** goal.status 变为 "achieved"，复查任务停止调度，但历史数据保留

#### Scenario: 归档目标
- **WHEN** 用户归档一个 inactive 超过 30 天的目标
- **THEN** goal.status 变为 "archived"，目标从默认列表中隐藏，可在归档视图中查看

### Requirement: 目标进度追踪
系统 SHALL 基于目标下所有关联 Wiki 页面的复习掌握率计算目标进度。进度 = avg(所有关联页面的 mastery) × 100，范围为 0-100。

#### Scenario: 进度自动更新
- **WHEN** 用户完成一次关于某知识点的复习后，mastery 更新
- **THEN** 关联目标的 progress 字段自动重新计算

### Requirement: Wiki 页面关联目标
系统 SHALL 支持手动和自动两种方式将 Wiki 页面关联到目标。每个 Wiki 页面最多自动关联 3 个目标。

#### Scenario: 手动关联
- **WHEN** 用户在 Wiki 页面点击"关联到目标"并从列表中选择目标
- **THEN** 创建 goal_wiki_links 记录，source = "manual"，is_new = 1

#### Scenario: 自动匹配关联
- **WHEN** 新 Wiki 页面编译完成且 AI 语义匹配得分 ≥ 0.6 或关键词重叠率 ≥ 30%
- **THEN** 自动创建 goal_wiki_links 记录，source = "auto"，is_new = 1，最多关联 3 个目标

#### Scenario: 新关联高亮
- **WHEN** 目标详情页打开且存在 is_new = 1 的关联
- **THEN** 显示"有 N 个新知识点"通知，展示后标记 is_new = 0
