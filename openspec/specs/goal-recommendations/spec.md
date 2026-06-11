## ADDED Requirements

### Requirement: AI 搜索推荐资源
创建目标后，系统 SHALL 调用 AI 联网搜索，生成由浅入深的推荐资源列表：入门级 2-3 篇、进阶级 3-5 篇、深入级 2-3 篇。

#### Scenario: 正常生成推荐
- **WHEN** 用户创建目标且 AI 搜索可用
- **THEN** 目标详情页展示推荐资源列表，按难度分层，每条含标题、来源 URL、摘要、难度标签

#### Scenario: 搜索不可用降级
- **WHEN** AI 搜索 API 不可用或超时（>30s）
- **THEN** 目标详情页显示"AI 搜索暂不可用"提示，用户可手动添加学习资源链接

### Requirement: 推荐操作
用户 SHALL 可以对每条推荐执行收录或忽略操作。收录后 SHALL 自动抓取内容并进入内容列表，随后编译为 Wiki 页面。

#### Scenario: 收录推荐
- **WHEN** 用户对某推荐点击"收录"
- **THEN** recommendation.status 变为 "imported"，内容自动抓取并创建 content 记录，关联 content.id 到 recommendation.imported_content_id

#### Scenario: 忽略推荐
- **WHEN** 用户对某推荐点击"忽略"
- **THEN** recommendation.status 变为 "dismissed"，该条从推荐列表隐藏

### Requirement: 现有 Wiki 关联检测
生成推荐的同时，系统 SHALL 扫描现有 Wiki 知识库，列出已有相关页面。

#### Scenario: 已有相关知识点
- **WHEN** 目标"掌握 Rust 所有权"下已有 2 个 Wiki 页面标题含"所有权"关键词
- **THEN** 推荐列表上方显示"已有 2 个相关知识点"，并列出页面链接
