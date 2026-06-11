## ADDED Requirements

### Requirement: 考试创建
系统 SHALL 基于单个目标创建考试，出题范围 SHALL 基于艾宾浩斯遗忘曲线圈定。默认优先出即将过期和已过期的知识点题目，辅以少量已掌握知识点。

#### Scenario: 系统推荐考试
- **WHEN** 目标下有 ≥ 5 个知识点的 review_schedule.next_review_at 在未来 3 天内
- **THEN** 学习首页显示考试建议卡片："目标「XXX」建议测一次"

#### Scenario: 用户主动发起考试
- **WHEN** 用户在目标详情页点击"开始考试"
- **THEN** 系统为该目标创建 exam 记录，status = "in_progress"，生成 20-30 道题目

### Requirement: 混合题型生成
考试 SHALL 由选择题（~50%）、判断题（~20%）、简答/论述（~30%）组成。每个题目 SHALL 关联一个 Wiki 页面作为出题来源。

#### Scenario: 考试题目组成
- **WHEN** 一次考试包含 20 题
- **THEN** 选择题 10 题、判断题 4 题、论述题 6 题

#### Scenario: 变换出题角度
- **WHEN** 某知识点的 mastery > 0.7
- **THEN** 系统 SHALL 对该知识点的出题增加难度偏移（反向提问、跨知识点关联、应用场景题），而非直接从 Wiki 原文生成

### Requirement: 考试中断恢复
系统 SHALL 支持考试中断后恢复。用户关闭窗口后重新打开，系统 SHALL 检测未完成的考试并提供"继续"或"重新开始"选项。

#### Scenario: 考试中断后继续
- **WHEN** 用户有一个 status = "in_progress" 且 started_at 在 24 小时内的考试
- **THEN** 打开学习 Tab 时弹窗："上次考试未完成，继续还是重新开始？"

#### Scenario: 超时考试处理
- **WHEN** 考试 started_at 超过 24 小时仍未完成
- **THEN** 自动标记为过期，用户只能重新开始

### Requirement: 考后反馈
考试完成后系统 SHALL 展示总分 + 等级（A/B/C/D）、逐题解析、薄弱诊断（答错知识点 + 错误类型分析）、趋势对比（与上次同目标考试对比）、学习建议。

#### Scenario: 首次考试无趋势对比
- **WHEN** 这是该目标的第一次考试
- **THEN** 趋势对比区域显示"完成下次考试后将看到进步趋势"而非留空

#### Scenario: 薄弱诊断
- **WHEN** 考试中 3 道关于"Rust 所有权"的题目答错
- **THEN** 诊断报告识别该知识点为薄弱点，错误类型标注为"理解偏差"，并自动建议重新学习
