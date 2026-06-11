## ADDED Requirements

### Requirement: 自动匹配触发
系统 SHALL 在以下事件发生时触发内容与目标的自动匹配:Wiki 页面编译完成、新目标创建。
原 spec 列出的"剪贴板内容保存 / 文件夹同步导入完成 / 手动添加内容"三个 content 阶段触发因尚无 `wiki_page_id` 而暂不实施,改由用户在内容列表手动编译时进入主路径。

#### Scenario: Wiki 编译完成后自动匹配
- **WHEN** 一个或多个 Wiki 页面通过 `compile_content_to_wiki` 编译完成
- **THEN** 系统对每个新编译的页面 spawn 后台任务,与所有 `status = "active"` 的目标进行匹配

#### Scenario: 新目标反向匹配
- **WHEN** 新目标通过 `create_goal` 创建完成
- **THEN** 系统对现有所有 Wiki 页面 spawn 后台扫描任务,反向匹配

### Requirement: 匹配算法(AI 主路径 + 关键词降级)
系统 SHALL 支持两级匹配:AI 自评分(LLM 在 prompt 内输出每个 goal 的 relevance_score,0~1)和关键词重叠匹配(`|wiki_terms ∩ goal_terms| / |goal_terms|`)。AI 优先,AI 失败/超时/未配置时自动降级到关键词匹配。两路径的阈值由 settings 项 `auto_link_sensitivity` 联动控制。

#### Scenario: AI 自评分成功
- **WHEN** AI 返回的 relevance_score 不低于当前阈值
- **THEN** 自动关联,relevance_score = AI 返回值

#### Scenario: 降级到关键词匹配
- **WHEN** AI 调用失败/超时/未配置
- **THEN** 切换关键词重叠计算,重叠率不低于当前阈值的自动关联,relevance_score = 重叠率

### Requirement: 关联上限
每个 Wiki 页面 SHALL 最多自动关联 3 个活跃目标,按 relevance_score 降序取 top-3。

#### Scenario: 匹配超过 3 个目标
- **WHEN** 一个 Wiki 页面与 5 个活跃目标匹配成功
- **THEN** 仅关联得分最高的 3 个目标,其余不创建关联记录
