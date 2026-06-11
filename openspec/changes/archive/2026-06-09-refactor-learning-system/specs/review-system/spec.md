## MODIFIED Requirements

### Requirement: 复习格式
复习系统 SHALL 支持 3 种格式：Quiz（选择题 4 选 1）、Cloze（填空）、Explain（简述）。RapidFire、Ordering、ErrorHunt 格式 SHALL 移除。

#### Scenario: 复习格式列表
- **WHEN** 用户开始一次复习会话
- **THEN** 仅使用 Quiz / Cloze / Explain 三种格式出题

#### Scenario: 旧格式数据保留
- **WHEN** 前端查询 review_logs
- **THEN** 仅查询 format IN ('quiz', 'cloze', 'explain') 的记录，旧格式数据保留在数据库中但不在前端展示

### Requirement: 复习范围
复习 SHALL 限定于目标下的知识点。非目标关联的 Wiki 页面不进入复习池。

#### Scenario: 复习池
- **WHEN** 系统计算今日待复习列表
- **THEN** 仅查询 goal_wiki_links 中关联的页面对应的 review_schedules

#### Scenario: 复习优先级
- **WHEN** 复习池中有多个待复习知识点
- **THEN** 排序优先级：1) overdue 优先 2) 活跃目标的优先于已归档目标 3) mastery 低的优先

### Requirement: 复习调度
系统 SHALL 复用现有 ReviewSchedule 机制和艾宾浩斯遗忘曲线算法（ease_factor / interval_days）。调度算法本身不做改动。

#### Scenario: 复习后更新调度
- **WHEN** 用户完成一次复习（任意格式）并作答
- **THEN** review_schedule 的 interval_days 按 ease_factor 调整，next_review_at 重新计算，mastery 更新

## ADDED Requirements

### Requirement: 复习入口
学习首页 SHALL 显示"今日待复习 N 个"卡片，一键开始连贯复习会话（逐题作答，不需手动选题）。

#### Scenario: 一键开始复习
- **WHEN** 用户在学习首页点击"开始复习"
- **THEN** 系统拉取今日待复习的所有题目，按优先级排序，逐题展示

#### Scenario: 复习进度
- **WHEN** 用户在复习会话中作答
- **THEN** 顶部显示进度条 "已完成 5/12"，每答完一题即时出反馈

## REMOVED Requirements

### Requirement: RapidFire 复习
**Reason**: 格式冗余。Quiz / Cloze / Explain 三种格式已覆盖快速回忆、概念理解、深层掌握的完整复习维度。
**Migration**: review_logs 中 format = 'rapid_fire' 的记录保留，数据不删除，前端不再查询此格式。

### Requirement: Ordering 排序
**Reason**: 格式冗余。排序题的使用频率低，教学价值被 Cloze 和 Explain 覆盖。
**Migration**: review_logs 中 format = 'ordering' 的记录保留，数据不删除。

### Requirement: ErrorHunt 找错
**Reason**: 格式冗余。AI 生成高质量陷阱题目的成本高，质量不稳定。
**Migration**: review_logs 中 format = 'error_hunt' 的记录保留，数据不删除。
