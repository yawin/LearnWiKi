## ADDED Requirements

### Requirement: 配置同步文件夹
系统 SHALL 允许用户在设置页面配置多个本地文件夹路径用于同步。每个路径可独立启用/禁用、删除。

#### Scenario: 添加同步文件夹
- **WHEN** 用户在设置页通过系统文件夹选择器选择一个路径并确认
- **THEN** 创建 sync_folders 记录，enabled = 1，路径不可重复

#### Scenario: 路径重叠检测
- **WHEN** 用户尝试添加的新路径是已有路径的子目录（或已有路径是新路径的子目录）
- **THEN** 系统拒绝添加并提示"此路径与已有同步文件夹重叠"

### Requirement: 手动触发同步
系统 SHALL 在内容列表页提供"同步"按钮，用户点击后 SHALL 扫描所有已启用的同步文件夹。

#### Scenario: 增量同步
- **WHEN** 用户点击"同步"且某文件 file_path + mtime 均未变
- **THEN** 该文件跳过，不重复导入

#### Scenario: 更新同步
- **WHEN** 某文件 file_path 存在但 mtime 已更新
- **THEN** 重新导入该文件，sync_record.status 标记为 "updated"

#### Scenario: 超大文件跳过
- **WHEN** 某文件大小超过 50MB
- **THEN** 跳过该文件，sync_record.status 标记为 "error"，记录错误原因，不阻塞其他文件处理

### Requirement: 文件格式支持
系统 SHALL 支持以下文件格式：.md / .txt / .pdf / .docx / .epub（文档类）、.png / .jpg / .jpeg / .webp（图片类）。

#### Scenario: 文档类文件处理
- **WHEN** 同步到 .md 或 .txt 文件
- **THEN** 提取文本内容，创建 content_type = "text" 的内容条目

#### Scenario: 二进制伪装检测
- **WHEN** .txt 文件前 4KB 包含 null byte
- **THEN** 判定为二进制文件，标记 error，不导入

### Requirement: 同步结果展示
同步完成后 SHALL 展示结果面板：新导入数量、更新数量、跳过数量，每项可展开查看详情。

#### Scenario: 同步完成
- **WHEN** 同步操作结束
- **THEN** 弹出结果面板，显示 "✅ 新导入 X 个 / 🔄 更新 Y 个 / ⏭️ 跳过 Z 个"，每个文件可点击查看

### Requirement: 与目标联动
文件夹同步导入的内容 SHALL 在编译为 Wiki 后触发目标匹配，命中则自动关联目标并加入学习列表。

#### Scenario: 端到端联动
- **WHEN** 用户同步了一个关于"机器学习"的 .md 文件
- **THEN** 文件导入内容列表 → 用户编译为 Wiki → 触发 content-goal-matching → 如果与目标"理解机器学习"匹配 → 自动关联 → 目标详情页出现新知识点标记
