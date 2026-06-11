# LearnWiki 代码规范

> 本文档定义 LearnWiki 项目的编码标准、约定和最佳实践。
> 所有贡献者应在开发前阅读并遵守。

---

## 1. 通用原则

### 1.1 命名规范总览

| 类型 | 规范 | 示例 |
|------|------|------|
| 文件名/目录名 | `kebab-case` | `wiki-page-detail.tsx` |
| TS/TSX 组件名 | `PascalCase` | `WikiPageDetail` |
| TS 函数/变量 | `camelCase` | `getWikiPage()` |
| TS 类型/接口 | `PascalCase` | `WikiPage` |
| Rust 文件/模块 | `snake_case` | `wiki_engine.rs` |
| Rust 结构体 | `PascalCase` | `WikiPage` |
| Rust 函数/方法 | `snake_case` | `get_wiki_page()` |
| SQL 表名 | `snake_case` | `wiki_pages` |
| SQL 字段 | `snake_case` | `last_compiled_at` |
| CSS 类名 | Tailwind（不使用自定义类名） | — |
| Git 分支 | `type/description` | `feat/reading-inbox` |
| 环境变量 | `UPPER_SNAKE_CASE` | `OPENWIKI_API_KEY` |

### 1.2 语言选择

| 场景 | 语言 |
|------|------|
| Rust 后端逻辑 | Rust |
| 前端 UI 组件 | TypeScript + React |
| 前端状态管理 | TypeScript + Zustand |
| 前端服务层 | TypeScript |
| 数据库迁移 | SQL |
| 配置文件 | YAML / TOML |
| 自动化脚本 | Shell / Python |

### 1.3 代码风格工具

```jsonc
// .prettierrc（前端）
{
  "semi": true,
  "singleQuote": false,
  "tabWidth": 2,
  "trailingComma": "all",
  "printWidth": 100
}
```

```toml
# rustfmt.toml（后端）
max_width = 100
tab_spaces = 4
edition = "2021"
```

---

## 2. 项目结构

### 2.1 顶层目录

```
learnwiki/
├── src/                    # 前端 React 代码
├── src-tauri/              # Rust 后端代码 + Tauri 配置
│   └── src/
│       ├── commands/       # Tauri 命令
│       ├── storage/        # 数据库和模型
│       │   ├── migrations/ # SQL 迁移文件
│       ├── ai/             # AI 客户端+引擎
│       ├── capture/        # 内容捕获
│       └── export/         # 导出
├── docs/                   # 设计文档
├── scripts/                # 自动化脚本
└── tests/                  # 端到端测试
```

### 2.2 前端目录规范

```
src/
├── features/               # 功能模块（每个一个目录）
│   ├── content-list/       # 内容列表模块
│   │   ├── ContentList.tsx
│   │   ├── ContentCard.tsx
│   │   └── ImagePreview.tsx
│   ├── wiki/               # Wiki 模块
│   │   ├── WikiView.tsx
│   │   ├── WikiPageDetail.tsx
│   │   └── WikiGraphView.tsx
│   └── learning/           # 👈 学习平台模块（新增）
│       ├── LearningView.tsx      # 主容器
│       ├── LearningDashboard.tsx # 仪表盘
│       ├── TaskBoard.tsx        # 任务看板
│       ├── TaskDetail.tsx       # 任务详情
│       ├── ReviewSession.tsx    # 复习页
│       └── ReadingInbox.tsx     # 阅读收件箱
├── components/             # 通用组件（跨模块复用）
│   └── BubbleView.tsx
├── services/               # API 调用层
│   ├── wikiService.ts
│   └── learningService.ts   # 👈 新增
├── stores/                 # Zustand 状态管理
│   ├── wikiStore.ts
│   └── learningStore.ts     # 👈 新增
├── types/                  # TypeScript 类型定义
│   ├── wiki.ts
│   ├── content.ts
│   └── learning.ts          # 👈 新增
└── App.tsx                 # 根组件 + 路由
```

**规则：**
- 每个功能模块一个目录，目录名 `kebab-case`
- 主组件文件名与组件名一致（`PascalCase`）
- 子组件放在同目录下，文件名 `PascalCase`
- 通用组件放在 `components/` 目录
- service/store/type 文件名 `camelCase`

---

## 3. TypeScript / React 规范

### 3.1 组件结构

每个组件遵循统一结构顺序：

```tsx
// 1. Imports（按顺序：React → 第三方 → 内部 → 类型）
import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { BookOpen, Trash2 } from "lucide-react";
import { useWikiStore } from "../../stores/wikiStore";
import { getWikiPage } from "../../services/wikiService";
import type { WikiPage } from "../../types/wiki";

// 2. 常量（如果有）
const PAGE_SIZE = 50;
const TYPE_ICONS = { concept: BookOpen, entity: BookOpen } as const;

// 3. Props 接口
interface WikiPageDetailProps {
  page: WikiPage;
  onClose: () => void;
  onDelete: (id: string) => void;
}

// 4. 组件函数
export function WikiPageDetail({ page, onClose, onDelete }: WikiPageDetailProps) {
  // 4a. Hooks（顶部）
  const { t } = useTranslation();
  const [sources, setSources] = useState([]);
  const [loading, setLoading] = useState(true);

  // 4b. Effects
  useEffect(() => {
    loadSources();
  }, [page.id]);

  // 4c. 事件处理函数（useCallback 包裹）
  const handleClose = useCallback(() => {
    onClose();
  }, [onClose]);

  // 4d. 辅助函数
  async function loadSources() { /* ... */ }

  // 4e. 条件渲染/空状态
  if (loading) return <LoadingSkeleton />;

  // 4f. 主渲染
  return (
    <div>...</div>
  );
}
```

### 3.2 Props 接口定义

```tsx
// ✅ 正确：显式定义 Props 接口
interface TaskCardProps {
  id: string;
  title: string;
  difficulty: "easy" | "medium" | "hard";
  onStatusChange?: (id: string, status: TaskStatus) => void;
}

// ❌ 错误：使用 inline 类型
export function TaskCard({ id, title, difficulty }: {
  id: string;           // ← 不清晰，无法复用
  title: string;
  difficulty: string;
})
```

### 3.3 状态管理（Zustand）

```tsx
// ✅ 正确：按功能模块拆分 Store
export const useLearningStore = create<LearningStore>()((set, get) => ({
  // 状态
  tasks: [],
  activeFilter: "all",
  isLoading: false,

  // 同步操作
  setActiveFilter: (filter) => set({ activeFilter: filter }),

  // 异步操作
  loadTasks: async () => {
    set({ isLoading: true });
    try {
      const data = await getTasksByStatus("all");
      set({ tasks: data, isLoading: false });
    } catch (e) {
      console.error("Failed to load tasks:", e);
      set({ isLoading: false });
    }
  },
}));

// 使用 selector 避免不必要的重渲染
const tasks = useLearningStore((s) => s.tasks);       // ✅
const { tasks, loadTasks } = useLearningStore();       // ❌ 会导致所有 subscriber 重渲染
```

### 3.4 样式（Tailwind CSS 4）

```tsx
// ✅ 正确：使用 Tailwind 原子类
<button
  className="flex items-center gap-1 px-3 py-1.5 text-[13px] font-medium
             rounded-lg transition-all duration-200
             bg-orange-500 text-white hover:bg-orange-600"
>
  导入
</button>

// ❌ 错误：自定义 CSS 类名（除非必须）
<button className="import-button">导入</button>

// ⚠️ 例外：只有在 Tailwind 无法表达时才使用内联 style
// 如动态宽度、计算值等
<div style={{ width: `${progress}%` }} />
```

### 3.5 国际化

```tsx
// ✅ 正确：使用 useTranslation hook
const { t } = useTranslation("learning");
return <h2>{t("dashboard.title")}</h2>;

// ❌ 错误：硬编码中文
return <h2>学习仪表盘</h2>;
```

### 3.6 错误处理

```tsx
// ✅ 正确：try-catch + 用户可见错误提示
try {
  await createTask(data);
  showSuccess("任务创建成功");
} catch (e) {
  console.error("创建任务失败:", e);
  showError("创建失败，请重试");
}

// ❌ 错误：吞掉错误
try {
  await createTask(data);
} catch (e) {
  // 什么都没做
}
```

---

## 4. Rust 后端规范

### 4.1 文件结构

```rust
// 每个文件按顺序组织：

// 1. Imports（标准库 → 第三方 → crate 内部）
use std::sync::Arc;
use chrono::Utc;
use tauri::State;
use crate::storage::database::Database;
use crate::storage::models::CapturedContent;

// 2. 常量
const MAX_RETRIES: u32 = 3;
const BACKUP_DIR: &str = "backups";

// 3. 结构体定义
pub struct AppState {
    pub db: Arc<Database>,
}

// 4. 公开函数（带 #[tauri::command]）
#[tauri::command]
pub fn create_learning_path(
    state: State<'_, AppState>,
    title: String,
) -> Result<LearningPath, String> {
    // ...
}

// 5. 私有辅助函数
fn validate_status_transition(from: &str, to: &str) -> bool {
    // ...
}
```

### 4.2 错误处理

```rust
// ✅ 正确：使用 Result 返回，错误信息有实际意义
pub fn get_wiki_page(id: &str) -> Result<WikiPage, String> {
    let conn = db.conn.lock().map_err(|e| format!("数据库锁错误: {}", e))?;
    // ...
    conn.query_row(...)
        .map_err(|e| format!("查询 Wiki 页面失败 (id={}): {}", id, e))
}

// ✅ 正确：使用 map_err 提供上下文
repo.find_content_by_hash(&hash)
    .map_err(|e| format!("去重查询失败: {}", e))?

// ❌ 避免：无信息的错误
Err("Error".to_string())
```

### 4.3 数据库访问模式

```rust
// ✅ 正确：使用 Repository 模式封装数据库操作
pub struct Repository {
    db: Arc<Database>,
}

impl Repository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn get_all_tasks(&self) -> Result<Vec<PracticeTask>, String> {
        let conn = self.db.conn.lock().map_err(|e| format!("{e}"))?;
        let mut stmt = conn.prepare("SELECT * FROM practice_tasks ORDER BY created_at DESC")
            .map_err(|e| format!("{e}"))?;
        let tasks = stmt.query_map([], |row| {
            Ok(PracticeTask {
                id: row.get("id")?,
                title: row.get("title")?,
                // ...
            })
        }).map_err(|e| format!("{e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("{e}"))?;
        Ok(tasks)
    }
}

// ❌ 避免：在 Tauri 命令中直接操作数据库
#[tauri::command]
pub fn bad_example(state: State<'_, AppState>) -> Result<(), String> {
    let conn = state.db.conn.lock().unwrap(); // ← 可能 panic
    conn.execute("...", []).unwrap();          // ← 可能 panic
    Ok(())
}
```

### 4.4 SQL 迁移规则

```sql
-- ✅ 正确：每个迁移文件自包含、幂等
-- migrations/017_add_learning_tables.sql

-- 每个迁移文件只做一件事：建表或加字段
CREATE TABLE IF NOT EXISTS learning_paths (
    id              TEXT PRIMARY KEY,
    title           TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 字段变更用 ALTER TABLE，不直接在原表上改
ALTER TABLE wiki_pages ADD COLUMN author_name TEXT;

-- ❌ 避免：删除或修改已有数据的迁移（在本地开发阶段可以，但上线后不行）
```

**迁移文件命名：** `NNN_description.sql`（NNN = 三位数字序号递增）

---

## 5. 组件设计原则

### 5.1 单一职责

```tsx
// ✅ 正确：一个组件做一件事
function TaskCard({ task }: { task: PracticeTask }) {
  // 只负责展示单个任务卡片
}

function TaskBoard({ tasks }: { tasks: PracticeTask[] }) {
  // 只负责管理任务卡片布局和状态
}

// ❌ 错误：TaskBoard 组件内部既管布局又渲染每个任务的细节 UI
```

### 5.2 组件大小限制

| 组件类型 | 建议行数 | 超过后建议 |
|---------|---------|-----------|
| 原子组件（Button、Badge） | ≤ 30 行 | 无需拆分 |
| 业务组件（TaskCard） | ≤ 80 行 | 提取子组件 |
| 页面组件（LearningDashboard） | ≤ 200 行 | 拆分区块 |
| 根组件（App.tsx） | ≤ 150 行 | 拆分布局 |

### 5.3 避免过度抽象

```tsx
// ✅ 正确的抽象度：2-3 个组件共同的部分才抽成通用组件
// Card 被 TaskCard、WikiCard、ContentCard 共用 → 抽
// LoadingSkeleton 被多处使用 → 抽
// 某个按钮只在 TaskDetail 用一次 → 不抽

// ❌ 过度抽象：每个 div 都封装成组件
<AppContainer>
  <PageLayout>
    <ContentArea>
      <SectionWrapper>
        <DataDisplay>
          <Title text="Hello" />  {/* 就一行字 */}
```

---

## 6. Git 工作流

### 6.1 分支命名

```
分支类型:
  feat/     → 新功能     feat/reading-inbox
  fix/      → 修复      fix/task-status-transition
  refactor/ → 重构      refactor/repository-pattern
  docs/     → 文档      docs/coding-standards
  chore/    → 杂项      chore/update-dependencies

格式: <type>/<简短英文描述>
描述使用 kebab-case
```

### 6.2 提交信息

```bash
# ✅ 正确
feat: add reading inbox UI with status filtering
fix: prevent invalid task status transition from todo to reviewed
refactor: extract database repository pattern
docs: add coding standards document
chore: bump tauri to v2.0.0

# ✅ 需要更多上下文时加 body
feat: implement SM-2 spaced repetition algorithm

Implements the core SM-2 algorithm for review scheduling.
- Calculate next interval based on quality feedback
- Adjust ease factor for long-term stabilization
- Cap maximum interval at 180 days

# ❌ 避免
update
fix bug
wip
asdf
```

### 6.3 PR 规范

```markdown
## PR 标题: [E-X-Y] Story 标题的简短描述

## 关联
- Epic: E-4（间隔重复复习系统）
- Story: E-4-2

## 变更内容
- 新增 ReviewSession 组件
- 实现 SM-2 算法核心逻辑
- 新增 review_schedule / review_log 数据表

## 测试
- [x] 单元测试覆盖 SM-2 算法
- [x] 验证状态流转合法路径
- [x] 手动测试复习页 UI

## 截图（如果涉及 UI 变更）
[截图]

## 注意事项
- 需要先运行新的数据库迁移
- 新增环境变量：无
```

### 6.4 提交频率

```
✅ 正确：每个 Story 一个或两个 commit
  1. feat: add reading inbox data model + CRUD
  2. feat: add reading inbox UI

❌ 避免：
  - 一个 commit 包含多个不相关的 Story
  - 一个 Story 拆成 20 个"save progress"commit
  - 包含未完成的代码（WIP）的 commit
```

---

## 7. 测试规范

### 7.0 核心原则：每个 Story 必须有测试

> 一条 Story 完成的定义（Definition of Done）：
> **代码写完 + 测试通过 + 测试覆盖所有 Acceptance Criteria**

不允许任何无测试的 Story 进入主干分支。

---

### 7.1 三层测试策略

| 层级 | 覆盖内容 | 每 Story 最少用例数 | 框架 |
|------|---------|-------------------|------|
| **Unit（后端）** | Rust 函数、算法、工具函数、Repository | ≥ 3 | `cargo test` |
| **Unit（前端）** | Zustand store、TS 工具函数、纯逻辑 | ≥ 2 | Vitest |
| **Integration（后端）** | Tauri 命令 + 数据库 | ≥ 2 | `cargo test`（in-memory DB） |
| **Integration（前端）** | 组件渲染 + 用户交互 | ≥ 1 | Vitest + Testing Library |
| **E2E** | 完整用户流程 | ≥ 1（关键路径） | 手动（后期 Playwright） |

**每个 Story 的 Acceptance Criteria 必须逐条映射到测试用例：**

```
Acceptance Criteria:
  AC1: Given 用户有多个学习路径，When 打开仪表盘，Then 显示所有路径
  AC2: Given 用户没有学习路径，When 打开仪表盘，Then 显示空状态

测试映射:
  ✅ Unit Test: store 初始化返回空列表 → loadPaths 后填充数据
  ✅ Unit Test: store 在空列表时 isLoading=false, tasks.length=0
  ✅ Integration: 渲染 <LearningDashboard tasks={[]} /> 显示空状态
  ✅ Integration: 渲染 <LearningDashboard tasks={mockData} /> 显示路径列表
```

---

### 7.2 Test Case 标准模板

每个 Story 的 Test Case 写在对应的测试文件中，格式如下：

```rust
// ==========================================================
// Test Case: E-4-1-TC-001
// Story:    E-4-1 复习数据模型与 SM-2 算法
// AC:       Given quality=2, When 计算, Then interval × 3.0
// ==========================================================
#[test]
fn test_sm2_quality_2_multiplies_interval_by_3() {
    // Arrange
    let ease_factor = 2.5;
    let interval = 7;
    let quality = 2; // "轻松"

    // Act
    let result = sm2_calculate(quality, ease_factor, interval);

    // Assert
    assert_eq!(result.interval, 21); // 7 × 3.0
    assert!((result.ease_factor - 2.5).abs() < f64::EPSILON);
}
```

```tsx
// ==========================================================
// Test Case: E-1-3-TC-001
// Story:    E-1-3 学习仪表盘（列表视图）
// AC:       Given 用户有学习路径, When 渲染, Then 显示路径
// ==========================================================
describe("E-1-3 学习仪表盘", () => {
  it("[TC-001] 有路径时显示路径列表", () => {
    // Arrange
    const mockPaths = [
      { id: "1", title: "RAG 入门", progress: 0.65 },
      { id: "2", title: "Prompt 工程", progress: 0.22 },
    ];

    // Act
    render(<LearningDashboard learningPaths={mockPaths} />);

    // Assert
    expect(screen.getByText("RAG 入门")).toBeInTheDocument();
    expect(screen.getByText("Prompt 工程")).toBeInTheDocument();
    expect(screen.getByText("65%")).toBeInTheDocument();
  });
});
```

**测试用例命名规则：** `[Epic-Story]-TC-NNN`

| 字段 | 示例 | 说明 |
|------|------|------|
| Epic-Story | `E-4-1` | 对应的 Story 编号 |
| TC | TC | Test Case 缩写 |
| NNN | 001 | 三位数序号，从 001 开始 |

---

### 7.3 Given-When-Then 三明治结构

所有测试用例必须遵循 **Arrange-Act-Assert** 模式，对应 BDD 的 **Given-When-Then**：

```rust
// ✅ 正确：清晰的三段分割
#[test]
fn test_calculate_mastery_high_score() {
    // ═══ Arrange ═══
    let schedule = ReviewSchedule {
        review_count: 5,
        interval_days: 30,
        ..default()
    };
    let logs = vec![
        ReviewLog { is_correct: Some(1), ..default() }, // 全对
        ReviewLog { is_correct: Some(1), ..default() },
        ReviewLog { is_correct: Some(1), ..default() },
    ];

    // ═══ Act ═══
    let mastery = calculate_mastery(&schedule, &logs);

    // ═══ Assert ═══
    assert!(mastery > 0.8);  // 高掌握度
    assert!(mastery <= 1.0); // 不超过上限
}

// ❌ 错误：Arrange 和 Act 混在一起
#[test]
fn test_calculate_mastery() {
    let schedule = ReviewSchedule { review_count: 5, interval_days: 30, ..default() };
    let logs = vec![ReviewLog { is_correct: Some(1), ..default() }];
    let mastery = calculate_mastery(&schedule, &logs);
    assert!(mastery > 0.5);
    // 分不清输入和输出，不知道每个变量的作用
}
```

---

### 7.4 测试数据工厂（Test Data Factory）

避免在每个测试中手动构造复杂对象，使用工厂函数：

```rust
// ✅ 正确：统一的测试数据工厂
// 放在 tests/common/ 或每个模块的 test_helpers.rs 中

#[cfg(test)]
pub mod factory {
    use crate::storage::models::*;

    pub fn create_mock_task(id: &str) -> PracticeTask {
        PracticeTask {
            id: id.to_string(),
            module_id: "mod-1".to_string(),
            title: format!("任务 {}", id),
            description: "测试描述".to_string(),
            difficulty: "medium".to_string(),
            status: "not_started".to_string(),
            estimated_minutes: 30,
            related_wiki_pages: "[]".to_string(),
            created_wiki_pages: "[]".to_string(),
            ..Default::default() // 未使用的字段用默认值
        }
    }

    pub fn create_mock_log(is_correct: bool) -> ReviewLog {
        ReviewLog {
            id: uuid::Uuid::new_v4().to_string(),
            is_correct: Some(is_correct as i32),
            quality: if is_correct { 1 } else { 0 },
            ..Default::default()
        }
    }
}
```

```tsx
// ✅ 前端也使用工厂模式
// src/features/learning/__tests__/factories.ts

export function createMockTask(overrides?: Partial<PracticeTask>): PracticeTask {
  const defaults: PracticeTask = {
    id: "task-1",
    title: "用 LangChain 实现基础 RAG",
    difficulty: "medium",
    status: "not_started",
    estimated_minutes: 45,
    related_wiki_pages: ["page-1"],
    created_at: "2025-06-01T00:00:00Z",
    updated_at: "2025-06-01T00:00:00Z",
  };
  return { ...defaults, ...overrides };
}

// 使用
const task = createMockTask({ difficulty: "hard" });
```

---

### 7.5 Rust 测试规范

#### 7.5.1 算法/纯函数测试

```rust
// 测试文件位置：与源码同文件，使用 cfg(test)
// 或独立文件：src/algorithm/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    // ═══ 边界值 ═══
    #[test]
    fn test_prioritize_backlog_empty_input() {
        let result = prioritize_backlog(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_prioritize_backlog_max_overflow() {
        let items = (0..100).map(|i| create_mock_item(i)).collect();
        let result = prioritize_backlog(items);
        assert_eq!(result.len(), 5); // 最多返回 5 个
    }

    // ═══ 典型场景 ═══
    #[test]
    fn test_prioritize_backlog_oldest_first() {
        let items = vec![
            create_mock_item_with_overdue(10), // 过期 10 天
            create_mock_item_with_overdue(3),  // 过期 3 天
        ];
        let result = prioritize_backlog(items);
        assert_eq!(result[0].overdue_days, 10); // 过期久的排前面
    }

    // ═══ 异常输入 ═══
    #[test]
    fn test_prioritize_backlog_negative_overdue() {
        let items = vec![create_mock_item_with_overdue(-1)]; // 还没过期
        let result = prioritize_backlog(items);
        assert!(result.is_empty()); // 不应出现在待处理中
    }
}
```

#### 7.5.2 Repository 测试（数据库）

```rust
// 使用内存数据库，不依赖真实文件

#[cfg(test)]
mod repository_tests {
    use super::*;
    use crate::storage::database::Database;

    fn setup_repo() -> (Repository, Arc<Database>) {
        let db = Arc::new(Database::new_in_memory().unwrap());
        let repo = Repository::new(db.clone());
        (repo, db)
    }

    #[test]
    fn test_create_and_get_learning_path() {
        // Arrange
        let (repo, _db) = setup_repo();

        // Act
        let path = repo.create_learning_path("从零掌握 RAG", "beginner").unwrap();

        // Assert
        assert_eq!(path.title, "从零掌握 RAG");
        assert_eq!(path.difficulty, "beginner");
        assert_eq!(path.completion_rate, 0.0);

        // Act 2: 查询
        let loaded = repo.get_learning_path(&path.id).unwrap();
        assert_eq!(loaded.title, path.title);
    }

    #[test]
    fn test_task_status_transition_valid() {
        let (repo, _db) = setup_repo();
        let task = repo.create_task("task-1", "mod-1", "测试").unwrap();
        assert_eq!(task.status, "not_started");

        // 合法流转
        repo.update_task_status("task-1", "in_progress").unwrap();
        let task = repo.get_task("task-1").unwrap();
        assert_eq!(task.status, "in_progress");

        // 重复流转到同一状态 — 允许
        repo.update_task_status("task-1", "in_progress").unwrap();
    }

    #[test]
    fn test_task_status_transition_invalid() {
        let (repo, _db) = setup_repo();
        repo.create_task("task-1", "mod-1", "测试").unwrap();

        // 非法流转：not_started → completed（跳过了 in_progress）
        let result = repo.update_task_status("task-1", "completed");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("非法状态流转"));
    }
}
```

#### 7.5.3 Tauri 命令测试

```rust
// 使用 tauri::test 模拟 State

#[cfg(test)]
mod command_tests {
    use super::*;
    use tauri::test::mock_state;

    #[test]
    fn test_create_learning_path_command() {
        let db = Arc::new(Database::new_in_memory().unwrap());
        let app_state = AppState { db: db.clone() };
        let state = mock_state(app_state);

        let result = create_learning_path(state, "测试路径".to_string());
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path.title, "测试路径");
    }
}
```

---

### 7.6 前端测试规范

#### 7.6.1 Store 测试

```tsx
// src/stores/__tests__/learningStore.test.ts

import { useLearningStore } from "../learningStore";

// 每个测试前重置 store 状态
beforeEach(() => {
  useLearningStore.setState({
    tasks: [],
    isLoading: false,
    error: null,
  });
});

describe("learningStore", () => {
  it("[TC-001] 初始状态为空", () => {
    const { tasks, isLoading } = useLearningStore.getState();
    expect(tasks).toEqual([]);
    expect(isLoading).toBe(false);
  });

  it("[TC-002] setActiveFilter 更新筛选条件", () => {
    useLearningStore.getState().setActiveFilter("completed");
    const { activeFilter } = useLearningStore.getState();
    expect(activeFilter).toBe("completed");
  });

  it("[TC-003] loadTasks 成功后填充数据", async () => {
    // Mock 服务层
    vi.spyOn(learningService, "getTasksByStatus").mockResolvedValue(mockTasks);

    await useLearningStore.getState().loadTasks();

    const { tasks, isLoading } = useLearningStore.getState();
    expect(tasks).toHaveLength(2);
    expect(isLoading).toBe(false);
  });

  it("[TC-004] loadTasks 失败时设置 error", async () => {
    vi.spyOn(learningService, "getTasksByStatus").mockRejectedValue(
      new Error("Network error")
    );

    await useLearningStore.getState().loadTasks();

    const { tasks, error, isLoading } = useLearningStore.getState();
    expect(tasks).toEqual([]);
    expect(error).toBe("Network error");
    expect(isLoading).toBe(false);
  });
});
```

#### 7.6.2 组件渲染测试

```tsx
// src/features/learning/__tests__/TaskBoard.test.tsx

import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { TaskBoard } from "../TaskBoard";

describe("TaskBoard", () => {
  const defaultTasks = [
    createMockTask({ id: "1", title: "任务 A", status: "not_started" }),
    createMockTask({ id: "2", title: "任务 B", status: "in_progress" }),
    createMockTask({ id: "3", title: "任务 C", status: "completed" }),
  ];

  it("[TC-001] 按状态分四列显示任务", () => {
    render(<TaskBoard tasks={defaultTasks} />);

    // 验证列标题
    expect(screen.getByText("待做")).toBeInTheDocument();
    expect(screen.getByText("进行中")).toBeInTheDocument();
    expect(screen.getByText("已完成")).toBeInTheDocument();
    expect(screen.getByText("已回顾")).toBeInTheDocument();

    // 验证任务在正确的列中
    const todoCol = screen.getByTestId("column-not_started");
    expect(within(todoCol).getByText("任务 A")).toBeInTheDocument();

    const inProgressCol = screen.getByTestId("column-in_progress");
    expect(within(inProgressCol).getByText("任务 B")).toBeInTheDocument();
  });

  it("[TC-002] 空列显示空状态", () => {
    render(<TaskBoard tasks={defaultTasks} />);

    // "已回顾"列没有数据
    const reviewedCol = screen.getByTestId("column-reviewed");
    expect(within(reviewedCol).getByText("暂无任务")).toBeInTheDocument();
  });

  it("[TC-003] 任务拖拽到新列触发状态更新", async () => {
    const onStatusChange = vi.fn();
    render(<TaskBoard tasks={defaultTasks} onStatusChange={onStatusChange} />);

    const taskCard = screen.getByText("任务 A");
    const targetColumn = screen.getByTestId("column-in_progress");

    // 使用 userEvent 模拟拖拽
    await userEvent.dragAndDrop(taskCard, targetColumn);

    expect(onStatusChange).toHaveBeenCalledWith("1", "in_progress");
  });
});
```

#### 7.6.3 空状态测试

```tsx
// 每个组件必须测试空状态

it("[TC-004] 无任务时显示空状态引导", () => {
  render(<TaskBoard tasks={[]} />);

  expect(screen.getByText("还没有任务")).toBeInTheDocument();
  expect(screen.getByText("导入知识后，AI 会自动推荐相关任务")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "创建第一个任务" })).toBeInTheDocument();
});
```

#### 7.6.4 服务层测试

```tsx
// src/services/__tests__/learningService.test.ts

import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("learningService", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("[TC-001] getTasksByStatus 调用后端并返回数据", async () => {
    (invoke as Mock).mockResolvedValue(mockTasks);

    const result = await getTasksByStatus("all");

    expect(invoke).toHaveBeenCalledWith("get_tasks_by_status", {
      status: "all",
    });
    expect(result).toHaveLength(2);
    expect(result[0].title).toBe("任务 A");
  });

  it("[TC-002] createTask 参数正确传递", async () => {
    (invoke as Mock).mockResolvedValue("new-task-id");

    const id = await createTask({
      title: "新任务",
      module_id: "mod-1",
      difficulty: "easy",
    });

    expect(invoke).toHaveBeenCalledWith("create_task", {
      title: "新任务",
      moduleId: "mod-1",
      difficulty: "easy",
    });
    expect(id).toBe("new-task-id");
  });
});
```

---

### 7.7 测试文件组织

```
src/
├── features/
│   └── learning/
│       ├── LearningDashboard.tsx
│       └── __tests__/              # 就近测试
│           ├── LearningDashboard.test.tsx
│           ├── TaskBoard.test.tsx
│           ├── factories.ts         # 测试数据工厂
│           └── fixtures.ts          # 固定测试数据 JSON
├── stores/
│   ├── learningStore.ts
│   └── __tests__/
│       └── learningStore.test.ts
├── services/
│   ├── learningService.ts
│   └── __tests__/
│       └── learningService.test.ts
└── types/
    └── learning.ts                  # 类型不需要单独测试

src-tauri/src/
├── commands/
│   ├── learning.rs
│   └── tests/                      # 独立测试目录（复杂命令）
│       └── learning_tests.rs
├── storage/
│   ├── repository.rs
│   └── repository.rs 中的 #[cfg(test)]  # 简单测试和源码一起
└── algorithm/
    ├── sm2.rs
    └── sm2.rs 中的 #[cfg(test)]          # 算法测试和源码一起
```

**规则：**
- 测试文件放在 `__tests__/` 目录下，与被测文件相邻
- 测试文件名 = `被测试文件名.test.tsx` 或 `被测试文件名.test.ts`
- Rust 简单测试和源码放一起（`#[cfg(test)] mod tests`）
- Rust 复杂测试放在独立目录 `tests/` 或 `模块名/tests/` 下
- 通用测试数据工厂放在 `factories.ts` 中

---

### 7.8 Mock 策略

| 场景 | 应该 Mock | 不应该 Mock |
|------|----------|------------|
| Store 测试 | 服务层 API 调用 | Zustand 内部状态管理逻辑 |
| 组件测试 | 服务层、后端 Tauri 命令 | 组件自身的渲染逻辑 |
| 服务层测试 | Tauri invoke | 数据转换和处理函数 |
| Rust Repository | 不需要 Mock（使用内存 DB） | 数据库查询本身 |
| Rust 算法 | 不需要 Mock | 算法计算本身 |

```tsx
// ✅ 正确：只在边界处 Mock
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

// ❌ 错误：Mock 组件内部
vi.mock("../TaskCard", () => ({ title }) => <div>{title}</div>);
// 这样测试 TaskBoard 时无法验证 TaskCard 的渲染
```

---

### 7.9 测试运行命令

```bash
# ─── 后端 ───
# 运行所有 Rust 测试
cargo test

# 运行特定模块的测试
cargo test test_sm2_

# 运行特定文件中的测试
cargo test --test integration_tests

# 打印测试输出（默认只显示失败的输出）
cargo test -- --nocapture

# ─── 前端 ───
# 运行所有前端测试
npx vitest

# 运行特定文件
npx vitest stores/__tests__/learningStore.test.ts

# UI 模式（可视化测试结果）
npx vitest --ui

# 覆盖率报告
npx vitest --coverage
```

#### CI 配置

```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo test

      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npx vitest run --coverage
```

---

### 7.10 每个 Story 的测试 Checklist

Story 合并前必须逐条检查：

```markdown
## 测试 Checklist

**Acceptance Criteria 覆盖:**
- [ ] AC1: 有对应的测试用例（TC-NNN）
- [ ] AC2: 有对应的测试用例
- [ ] AC3: 有对应的测试用例
- [ ] 所有 AC 都有测试覆盖

**测试类型覆盖:**
- [ ] Unit test（算法/逻辑）
- [ ] Integration test（组件渲染/命令执行）
- [ ] 空状态/边界条件测试
- [ ] 错误处理测试（网络失败、非法输入）

**代码覆盖:**
- [ ] 新增代码覆盖率 ≥ 80%
- [ ] 核心逻辑（算法、状态流转）覆盖率 100%

**测试结果:**
- [ ] `cargo test` 全部通过
- [ ] `npx vitest run` 全部通过
- [ ] 没有 flaky test（重复运行 3 次都通过）

**测试质量:**
- [ ] 每个测试使用 Given-When-Then / Arrange-Act-Assert
- [ ] 测试命名清晰（`test_<功能>_<场景>`）
- [ ] 没有测试间依赖（可独立运行）
- [ ] 没有 connect 到外部服务的测试
```

---

### 7.11 测试覆盖率要求

| 模块 | 最低覆盖率 | 测试重点 |
|------|-----------|---------|
| SM-2 算法 | **100%** | 所有 quality 值、边界间隔、ease factor 调整 |
| 任务状态流转 | **100%** | 所有合法/非法流转路径 |
| 积压优先级算法 | **100%** | 空列表、单元素、多元素、负值输入 |
| 掌握度计算 | **100%** | 各种复习次数和答对率组合 |
| 数据库 Repository | ≥ 80% | CRUD 操作、唯一约束、外键约束 |
| Tauri 命令 | ≥ 70% | 参数传递、错误返回、正常返回 |
| Zustand Store | ≥ 80% | 初始状态、操作后状态、异步加载成功/失败 |
| React 组件 | ≥ 60% | 正常渲染、空状态、交互事件 |
| 服务层 | ≥ 70% | 参数转换、调用后端、错误处理 |

---

### 7.12 禁止的测试模式

```rust
// ❌ 禁止 1：测试依赖外部服务
#[test]
fn test_search_web() {
    let result = search_api("RAG 2025").unwrap(); // 需要网络
}

// ❌ 禁止 2：测试依赖共享状态
#[test]
fn test_create_task() {
    create_task("测试").unwrap(); // 依赖全局 ID 生成器
}
#[test]
fn test_get_task() {
    let task = get_task("test-id").unwrap(); // 依赖前一个测试创建的数据
}

// ❌ 禁止 3：无断言的测试
#[test]
fn test_sm2_calculate() {
    let result = sm2_calculate(1, 2.5, 7);
    // 没有 assert！
}

// ❌ 禁止 4：过度 Mock
#[test]
fn test_add() {
    // 连加法函数都 Mock
    let add = mock_add(1, 2);
    assert_eq!(add, 3); // 测试的是 Mock 本身
}
```

```tsx
// ❌ 禁止 5：快照测试用于大型组件
it("matches snapshot", () => {
  const { container } = render(<LearningDashboard />);
  expect(container).toMatchSnapshot(); // 1000 行 HTML，没人 review 变化
});
// ✅ 改为：用 getByText / getByTestId 精确断言

// ❌ 禁止 6：测试实现细节
it("calls setState internally", () => {
  const setStateSpy = vi.spyOn(store, "setState");
  render(<TaskBoard />);
  expect(setStateSpy).toHaveBeenCalled(); // 测试实现而非行为
});
```

---

### 7.13 测试命名速查表

| 测试类型 | Rust 命名 | TS 命名 |
|---------|----------|---------|
| 正常路径 | `test_<fn>_happy_path` | `handles normal input correctly` |
| 边界值 | `test_<fn>_empty_input` | `handles empty input` |
| 边界值 | `test_<fn>_max_input` | `handles maximum input` |
| 错误处理 | `test_<fn>_invalid_input` | `handles invalid input` |
| 错误处理 | `test_<fn>_network_failure` | `handles network failure` |
| 并发 | `test_<fn>_concurrent_calls` | `handles concurrent calls` |
| 幂等 | `test_<fn>_idempotent` | `is idempotent when called twice` |

---

## 8. 文档规范

### 8.1 代码注释

```rust
// ✅ 好的注释：解释为什么（Why），而不是是什么（What）

// 使用 SM-2 算法的质量值选择：
// - 0（忘了）：重置间隔，降低难度系数
// - 1（记得）：正常递增间隔
// - 2（轻松）：快速递增间隔，增加难度系数
fn calculate_quality_feedback(quality: u8) -> (f64, f64) {
    // ...
}

// ❌ 无意义的注释：重复代码本身
// 设置标题
title = "Hello";
```

### 8.2 Rust Docstring

```rust
/// Wiki page compiled from captured content.
///
/// Each WikiPage represents a knowledge node in the knowledge graph.
/// It can be created by:
/// - AI auto-compiling from captured content
/// - User manually creating from task reflections
///
/// # Fields
/// - `source_task_id`: If this page was created from a task reflection,
///   this field links back to the source task.
pub struct WikiPage {
    // ...
}
```

### 8.3 设计文档

所有设计决策应记录在 `docs/` 目录下：

```
docs/
├── learning-platform-design.md  # 产品设计（已存在）
├── learning-platform-backlog.md # 敏捷需求（已存在）
├── coding-standards.md          # 代码规范（本文档）
└── architecture.md              # 架构决策（可选）
```

---

## 9. TypeScript 类型优先级

```tsx
// ✅ 1️⃣ interface：定义对象结构（首选）
interface PracticeTask {
  id: string;
  title: string;
  difficulty: "easy" | "medium" | "hard";
}

// ✅ 2️⃣ type：定义联合类型、元组、函数签名
type TaskStatus = "not_started" | "in_progress" | "completed" | "reviewed";
type Callback = (id: string) => void;

// ✅ 3️⃣ as const：定义常量映射
const DIFFICULTY_COLORS = {
  easy: "#16A34A",
  medium: "#CA8A04",
  hard: "#DC2626",
} as const;
// 自动推断类型: { readonly easy: "#16A34A"; readonly medium: "#CA8A04"; ... }
```

---

## 10. 性能注意事项

```tsx
// ✅ 正确：使用 useCallback / useMemo 避免不必要的重渲染
const handleClick = useCallback(() => {
  onSelect(task.id);
}, [task.id, onSelect]);

const sortedTasks = useMemo(() => {
  return [...tasks].sort((a, b) => a.difficulty.localeCompare(b.difficulty));
}, [tasks]);

// ✅ 正确：Zustand selector 精确取值
const taskCount = useLearningStore((s) => s.tasks.length);  // 只监听 length

// ❌ 避免：大列表无限制渲染
{tasks.map((task) => <TaskCard key={task.id} task={task} />)}
// → 使用虚拟列表（react-window）处理 100+ 的任务列表

// ❌ 避免：在渲染循环中执行计算
{tasks.map((task) => {
  const score = calculateComplexScore(task); // 每次渲染都执行
  return <TaskCard key={task.id} score={score} />;
})}
// → 提前计算或使用 useMemo
```

---

## 11. 数据库查询性能

```sql
-- ✅ 正确：需要什么字段查什么
SELECT id, title, status, next_review_at FROM review_schedule WHERE next_review_at <= datetime('now');

-- ❌ 避免：SELECT *
SELECT * FROM review_schedule;
```

```rust
// ✅ 正确：分页查询
pub fn get_all_tasks(&self, limit: i64, offset: i64) -> Result<Vec<PracticeTask>, String> {
    let conn = self.db.conn.lock().map_err(|e| format!("{e}"))?;
    let mut stmt = conn.prepare("SELECT id, title, status, difficulty FROM practice_tasks ORDER BY created_at DESC LIMIT ?1 OFFSET ?2")
        .map_err(|e| format!("{e}"))?;
    // ...
}

// ✅ 正确：频繁查询的字段加索引
// review_schedule(next_review_at) ← 每日查询谁到期了
// practice_tasks(status)          ← 看板按状态筛选
```

---

## 12. 依赖管理

```bash
# ✅ 前端：锁定版本
npm install zustand@4.5.0    # ✅ 指定版本
npm install zustand          # ❌ 不指定版本，可能引入破坏性更新

# ✅ Rust：明确 features
rusqlite = { version = "0.32", features = ["bundled", "functions"] }
```

**依赖添加流程：**
1. 评估是否真的需要（能否用已有工具实现？）
2. 检查包的大小、维护状态、License
3. 锁定版本号
4. 在 PR 描述中说明引入该依赖的原因

---

## 13. 安全注意事项

```rust
// ✅ 正确：验证用户输入
fn validate_task_title(title: &str) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("任务标题不能为空".to_string());
    }
    if title.len() > 200 {
        return Err("任务标题不能超过 200 个字符".to_string());
    }
    Ok(())
}

// ✅ 正确：SQLite 参数化查询
conn.execute(
    "INSERT INTO practice_tasks (id, title) VALUES (?1, ?2)",
    rusqlite::params![id, title],
)?;

// ❌ 避免：SQL 拼接
let sql = format!("INSERT INTO tasks (title) VALUES ('{}')", title); // SQL 注入风险
```

---

## 14. 最后：代码审查 Checklist

每个 PR 提交前对照检查：

- [ ] 代码通过 `cargo check` / `npm run build`
- [ ] 无 `console.log` / `println!` / `dbg!` 残留
- [ ] 无 TODO / FIXME / HACK 注释残留（除非有 Issue 链接）
- [ ] 新增功能有对应测试
- [ ] 组件 Props 有明确的接口定义
- [ ] 错误信息有实际意义（不是 `"Error"`）
- [ ] SQL 使用参数化查询
- [ ] 迁移文件遵循 `NNN_description.sql` 命名
- [ ] 无硬编码的字符串（用 i18n 或常量代替）
- [ ] 提交信息符合规范
