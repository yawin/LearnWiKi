## ADDED Requirements

### Requirement: 自动关联敏感度设置
系统 SHALL 提供 `auto_link_sensitivity` settings 项,枚举值 `"loose" | "balanced" | "strict"`,默认 `"balanced"`。该设置 SHALL 同时控制内容-目标自动匹配中的 AI 阈值和关键词重叠阈值。

| 档位 | AI 阈值 | 关键词阈值 |
|---|---|---|
| `loose` | 0.5 | 0.2 |
| `balanced`(默认) | 0.6 | 0.3 |
| `strict` | 0.7 | 0.4 |

#### Scenario: 用户切换敏感度
- **WHEN** 用户在设置页"学习"分组切换到 `strict`
- **THEN** settings 表中 `auto_link_sensitivity` 写入 `"strict"`,后续触发的匹配使用 AI 阈值 0.7 / 关键词阈值 0.4

#### Scenario: AI 未配置时
- **WHEN** 用户未配置 AI API key
- **THEN** 该设置项仍可调(影响关键词阈值),界面 SHALL 显示提示文案告知当前用关键词兜底
