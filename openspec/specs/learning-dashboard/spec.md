## MODIFIED Requirements

### Requirement: 学习首页布局
LearningDashboard SHALL 展示三个核心区域：今日复习卡片（顶部）、我的目标列表（中部）、考试建议（底部）。不再展示课程地图（LearningPath）和模块列表（Module）。

#### Scenario: 首页默认视图
- **WHEN** 用户打开学习 Tab
- **THEN** 展示"今日待复习 N 个"卡片（最显眼位置）、下方为目标列表（每个目标显示进度条 + 状态）、底部为考试建议（如有）

#### Scenario: 无目标时的空状态
- **WHEN** 用户尚未创建任何目标
- **THEN** 显示引导卡片"设定你的第一个学习目标"，点击跳转到创建目标页面，不显示旧的学习路径入口

### Requirement: 新知识点通知
系统 SHALL 在首页顶部显示新关联通知："有 N 个新知识点加入了你的目标「XXX」"，点击跳转到对应目标详情页。

#### Scenario: 新内容自动关联通知
- **WHEN** 一条新内容自动匹配到目标"掌握 Rust"
- **THEN** 学习首页顶部出现非阻断通知，点击跳转到目标详情页展示新的知识点列表

## ADDED Requirements

### Requirement: 复习完成后返回
复习会话结束后 SHALL 展示简短的复习小结（正确率 + 用时），然后返回首页。

#### Scenario: 复习小结
- **WHEN** 用户完成当日所有复习题目
- **THEN** 展示小结卡片"完成 12 题，正确 9 题（75%），用时 8 分钟"，点击"返回首页"关闭

### Requirement: 目标创建快捷入口
首页 SHALL 提供醒目的"创建目标"入口。

#### Scenario: 从首页创建目标
- **WHEN** 用户在首页点击"+ 新目标"按钮
- **THEN** 跳转到 GoalCreate 页面

## REMOVED Requirements

### Requirement: LearningPath 展示
**Reason**: LearningPath + Module 双层体系被 Goal 替代。目标直接关联 Wiki 页面，不需要中间层级。
**Migration**: LearningPath 数据保留（不删除），前端不再展示和查询。

### Requirement: Module 列表展示  
**Reason**: Module 是 LearningPath 的子单位，与 Goal → Wiki 的扁平关联模型冲突。
**Migration**: Module 数据保留，前端不再展示。

### Requirement: TaskBoard 入口
**Reason**: PracticeTask 任务管理被目标学习替代。
**Migration**: PracticeTask 数据保留，TaskBoard UI 移除。
