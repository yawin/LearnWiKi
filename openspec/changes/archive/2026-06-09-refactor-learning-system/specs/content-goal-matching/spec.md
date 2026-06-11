## ADDED Requirements

### Requirement: 自动匹配触发
系统 SHALL 在以下事件发生时触发内容与目标的自动匹配：剪贴板内容保存、文件夹同步导入完成、手动添加内容、Wiki 页面编译完成、新目标创建。

#### Scenario: 内容保存后自动匹配
- **WHEN** 一条新内容通过剪贴板保存
- **THEN** 系统对该内容与所有 status = "active" 的目标进行匹配，匹配成功的自动创建 goal_wiki_links

#### Scenario: 新目标反向匹配
- **WHEN** 新目标创建完成
- **THEN** 系统对现有所有 Wiki 页面进行反向匹配

### Requirement: 匹配算法
系统 SHALL 支持两级匹配：AI 语义相似度匹配（cosine similarity ≥ 0.6）和关键词重叠匹配（目标 keywords 与页面标题/tags 的重叠率 ≥ 30%）。AI 匹配优先，AI 不可用时自动降级到关键词匹配。

#### Scenario: AI 语义匹配成功
- **WHEN** AI 返回的内容-目标相似度 ≥ 0.6
- **THEN** 自动关联，relevance_score = AI 返回的相似度值

#### Scenario: 降级到关键词匹配
- **WHEN** AI 匹配调用失败或超时
- **THEN** 降级为关键词重叠计算，重叠率 ≥ 30% 的自动关联，relevance_score = 重叠率

### Requirement: 关联上限
每个内容/Wiki 页面 SHALL 最多自动关联 3 个活跃目标，按 relevance_score 降序取 top-3。

#### Scenario: 匹配超过 3 个目标
- **WHEN** 一个 Wiki 页面与 5 个活跃目标匹配成功
- **THEN** 仅关联得分最高的 3 个目标，其余目标不创建关联记录
