// =====================================================================
// OpenWiki Mock Data — 完整中文数据集
// 所有时间戳用相对日期（daysAgo / daysLater）计算
// =====================================================================

// ===== 工具函数：生成相对时间 =====
export function daysAgo(n: number): string {
  const d = new Date();
  d.setDate(d.getDate() - n);
  return d.toISOString();
}
export function daysLater(n: number): string {
  const d = new Date();
  d.setDate(d.getDate() + n);
  return d.toISOString();
}
export function today(): string {
  return new Date().toISOString();
}

// ===================================================================
// 1. 学习路径 (LearningPath) × 3
// ===================================================================
export const learningPaths = [
  {
    id: 'path-frontend-adv',
    title: '前端工程化进阶',
    description: '从组件设计到构建工具链，系统掌握现代前端工程化最佳实践，涵盖 Monorepo、微前端、CI/CD 等核心主题。',
    topic: '前端开发',
    difficulty: 'advanced',
    estimated_days: 45,
    module_count: 4,
    completion_rate: 0.35,
    is_active: true,
    created_at: daysAgo(30),
    updated_at: daysAgo(1),
  },
  {
    id: 'path-rust-sys',
    title: 'Rust 系统编程入门',
    description: '从所有权系统到异步运行时，循序渐进掌握 Rust 系统编程，适合有其他语言基础的开发者。',
    topic: '系统编程',
    difficulty: 'intermediate',
    estimated_days: 60,
    module_count: 3,
    completion_rate: 0.15,
    is_active: true,
    created_at: daysAgo(20),
    updated_at: daysAgo(2),
  },
  {
    id: 'path-ml-basics',
    title: '机器学习基础',
    description: '从线性回归到神经网络，理解核心算法原理并动手实现，配套 Python 实践项目。',
    topic: '人工智能',
    difficulty: 'beginner',
    estimated_days: 90,
    module_count: 3,
    completion_rate: 0.0,
    is_active: false,
    created_at: daysAgo(60),
    updated_at: daysAgo(15),
  },
] as const;

// ===================================================================
// 2. 模块 (Module) — 每个路径 2-4 个
// ===================================================================
export const modules = [
  // ---- 前端工程化进阶 ----
  {
    id: 'mod-fe-components',
    path_id: 'path-frontend-adv',
    title: '组件设计模式',
    sort_order: 1,
    description: '深入理解 React 组件设计模式：组合、高阶组件、Render Props、Hooks 模式与状态管理。',
    theory_markdown: `# 组件设计模式

## 组件组合 (Composition)

组合是 React 的核心思想之一。通过组合而非继承来复用组件逻辑。

\`\`\`tsx
function Panel({ header, children, footer }: PanelProps) {
  return (
    <div className="panel">
      <div className="panel-header">{header}</div>
      <div className="panel-body">{children}</div>
      {footer && <div className="panel-footer">{footer}</div>}
    </div>
  );
}
\`\`\`

## 高阶组件 (HOC)

HOC 是一个函数，接受一个组件并返回一个新组件。
`,
    reading_list_json: JSON.stringify([
      { title: 'React 官方文档 — 组合 vs 继承', url: 'https://react.dev/' },
      { title: 'Advanced React Patterns', url: 'https://example.com/patterns' },
    ]),
    estimated_read_minutes: 45,
    discussion_prompts: '1. 组合和继承的区别在哪里？\n2. Hooks 能否完全替代 HOC？',
    community_solutions: '常见方案见 solutions 目录',
    task_ids: 'task-fe-composition,task-fe-hoc,task-fe-renderprops',
    status: 'completed',
    completed_at: daysAgo(3),
    created_at: daysAgo(30),
    updated_at: daysAgo(3),
  },
  {
    id: 'mod-fe-build',
    path_id: 'path-frontend-adv',
    title: '构建工具链',
    sort_order: 2,
    description: '从 Webpack 到 Vite/Turbopack，掌握现代前端构建工具的原理与配置优化。',
    theory_markdown: `# 构建工具链

## 模块打包原理

Webpack 通过入口文件递归构建依赖图，将各种资源通过 loader 和 plugin 处理。

## Vite 的优势

基于 ESM 的开发服务器实现秒级冷启动。
`,
    reading_list_json: JSON.stringify([
      { title: 'Vite 官方文档', url: 'https://vitejs.dev/' },
    ]),
    estimated_read_minutes: 60,
    discussion_prompts: '1. 为什么 Vite 比 Webpack 快？\n2. Tree Shaking 的原理是什么？',
    community_solutions: '',
    task_ids: 'task-fe-vite,task-fe-webpack',
    status: 'in_progress',
    completed_at: null,
    created_at: daysAgo(25),
    updated_at: daysAgo(1),
  },
  {
    id: 'mod-fe-monorepo',
    path_id: 'path-frontend-adv',
    title: 'Monorepo 架构',
    sort_order: 3,
    description: '使用 Turborepo / Nx 搭建 Monorepo 工程，管理多包项目与共享配置。',
    theory_markdown: `# Monorepo 架构\n\n## 什么是 Monorepo\n\n在同一个仓库中管理多个项目/包。`,
    reading_list_json: JSON.stringify([]),
    estimated_read_minutes: 40,
    discussion_prompts: 'Monorepo vs Multi-repo 的优缺点？',
    community_solutions: '',
    task_ids: 'task-fe-monorepo',
    status: 'locked',
    completed_at: null,
    created_at: daysAgo(20),
    updated_at: daysAgo(20),
  },
  {
    id: 'mod-fe-micro',
    path_id: 'path-frontend-adv',
    title: '微前端实践',
    sort_order: 4,
    description: '基于 Module Federation 的微前端架构设计与落地。',
    theory_markdown: `# 微前端\n\n微前端将单体前端拆分为多个独立部署的小型应用。`,
    reading_list_json: JSON.stringify([]),
    estimated_read_minutes: 50,
    discussion_prompts: '如何解决微前端的样式隔离问题？',
    community_solutions: '',
    task_ids: 'task-fe-micro',
    status: 'locked',
    completed_at: null,
    created_at: daysAgo(15),
    updated_at: daysAgo(15),
  },

  // ---- Rust 系统编程 ----
  {
    id: 'mod-rust-ownership',
    path_id: 'path-rust-sys',
    title: '所有权与借用',
    sort_order: 1,
    description: '理解 Rust 的核心概念：所有权、借用、生命周期，以及如何写出安全的代码。',
    theory_markdown: `# 所有权系统

## 所有权规则

1. 每个值有且仅有一个所有者
2. 当所有者离开作用域，值被丢弃
3. 所有权可以转移（move）或借用（borrow）
`,
    reading_list_json: JSON.stringify([
      { title: 'The Rust Book — Ownership', url: 'https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html' },
    ]),
    estimated_read_minutes: 55,
    discussion_prompts: '1. 试比较 Rust 所有权与 GC 的优劣\n2. 什么场景需要 RefCell？',
    community_solutions: '',
    task_ids: 'task-rust-ownership,task-rust-borrow',
    status: 'in_progress',
    completed_at: null,
    created_at: daysAgo(20),
    updated_at: daysAgo(1),
  },
  {
    id: 'mod-rust-concurrency',
    path_id: 'path-rust-sys',
    title: '并发编程',
    sort_order: 2,
    description: '掌握 Send/Sync trait、消息传递、共享状态与 Tokio 异步运行时。',
    theory_markdown: `# 并发编程\n\nRust 的类型系统在编译期排除数据竞争。`,
    reading_list_json: JSON.stringify([]),
    estimated_read_minutes: 70,
    discussion_prompts: 'async/await 与 OS 线程的适用场景？',
    community_solutions: '',
    task_ids: 'task-rust-concurrency',
    status: 'locked',
    completed_at: null,
    created_at: daysAgo(15),
    updated_at: daysAgo(15),
  },
  {
    id: 'mod-rust-ffi',
    path_id: 'path-rust-sys',
    title: 'FFI 与 unsafe',
    sort_order: 3,
    description: '安全地调用 C 语言库，编写 unsafe 代码以及在 Rust 中嵌入其他语言。',
    theory_markdown: `# FFI\n\nRust 通过 extern "C" 块声明外部函数接口。`,
    reading_list_json: JSON.stringify([]),
    estimated_read_minutes: 50,
    discussion_prompts: 'unsafe 有哪些必要的使用场景？',
    community_solutions: '',
    task_ids: '',
    status: 'locked',
    completed_at: null,
    created_at: daysAgo(10),
    updated_at: daysAgo(10),
  },

  // ---- 机器学习基础 ----
  {
    id: 'mod-ml-linear',
    path_id: 'path-ml-basics',
    title: '线性回归与逻辑回归',
    sort_order: 1,
    description: '从最小二乘法到梯度下降，理解监督学习的两个基础模型。',
    theory_markdown: `# 线性回归\n\n线性回归假设目标值与特征之间存在线性关系。`,
    reading_list_json: JSON.stringify([]),
    estimated_read_minutes: 50,
    discussion_prompts: '逻辑回归为什么叫"回归"却做分类？',
    community_solutions: '',
    task_ids: 'task-ml-linear',
    status: 'available',
    completed_at: null,
    created_at: daysAgo(60),
    updated_at: daysAgo(15),
  },
  {
    id: 'mod-ml-tree',
    path_id: 'path-ml-basics',
    title: '决策树与集成方法',
    sort_order: 2,
    description: '从 ID3 到 XGBoost，理解树模型和集成学习的核心思想。',
    theory_markdown: `# 决策树\n\n决策树通过如果-否则规则对数据进行分类或回归。`,
    reading_list_json: JSON.stringify([]),
    estimated_read_minutes: 60,
    discussion_prompts: 'Bagging 与 Boosting 的主要区别？',
    community_solutions: '',
    task_ids: '',
    status: 'locked',
    completed_at: null,
    created_at: daysAgo(55),
    updated_at: daysAgo(55),
  },
  {
    id: 'mod-ml-nn',
    path_id: 'path-ml-basics',
    title: '神经网络入门',
    sort_order: 3,
    description: '从感知机到多层神经网络，理解反向传播与梯度消失。',
    theory_markdown: `# 神经网络\n\n神经网络由输入层、隐藏层和输出层组成。`,
    reading_list_json: JSON.stringify([]),
    estimated_read_minutes: 70,
    discussion_prompts: 'ReLU 为什么比 Sigmoid 更好？',
    community_solutions: '',
    task_ids: '',
    status: 'locked',
    completed_at: null,
    created_at: daysAgo(50),
    updated_at: daysAgo(50),
  },
];

// ===================================================================
// 3. 任务 (PracticeTask) × 12 — todo(3)/in_progress(3)/done(3)/blocked(3)
// ===================================================================
export const practiceTasks = [
  // ---- done (status: completed) ----
  {
    id: 'task-fe-composition',
    module_id: 'mod-fe-components',
    title: '实现 Panel 组合组件',
    description: '使用 children/header/footer 插槽实现一个通用的 Panel 组件，支持折叠、自定义样式。',
    difficulty: 'easy',
    estimated_minutes: 30,
    prerequisites: '了解 React JSX 和 Props',
    hint_content: '考虑使用 React.Children.map 来处理多个子元素。',
    reference_links: '{"docs":"https://react.dev/reference/react/Children"}',
    status: 'completed',
    started_at: daysAgo(7),
    completed_at: daysAgo(4),
    attempt_count: 1,
    is_starred: false,
    reflection: '初步理解了组合模式比继承更灵活。',
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: 'wiki-comp-patterns',
    related_wiki_pages: 'wiki-comp-patterns',
    tags: '["["react"",""composition"",""components"]"]',
    created_at: daysAgo(30),
    updated_at: daysAgo(4),
  },
  {
    id: 'task-fe-hoc',
    module_id: 'mod-fe-components',
    title: '实现 withLogger HOC',
    description: '编写一个高阶组件 withLogger，在组件挂载和更新时自动打印日志。',
    difficulty: 'medium',
    estimated_minutes: 25,
    prerequisites: '理解 HOC 模式',
    hint_content: '别忘了传递 ref 和 displayName。',
    reference_links: null,
    status: 'completed',
    started_at: daysAgo(6),
    completed_at: daysAgo(3),
    attempt_count: 2,
    is_starred: true,
    reflection: 'HOC 的 ref 传递是个坑，用 forwardRef 解决。',
    code_snippets: `function withLogger(Wrapped) {\n  return React.forwardRef((props, ref) => {\n    console.log('render:', Wrapped.displayName);\n    return <Wrapped ref={ref} {...props} />;\n  });\n}`,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["react","hoc","patterns"]',
    created_at: daysAgo(28),
    updated_at: daysAgo(3),
  },
  {
    id: 'task-rust-ownership',
    module_id: 'mod-rust-ownership',
    title: '实现自定义 String 类型',
    description: '仿照标准库的 String 实现基础的 new/push/pop 方法，实践所有权语义。',
    difficulty: 'medium',
    estimated_minutes: 45,
    prerequisites: '理解栈与堆的区别',
    hint_content: '注意 drop 方法的实现，防止内存泄漏。',
    reference_links: null,
    status: 'completed',
    started_at: daysAgo(5),
    completed_at: daysAgo(2),
    attempt_count: 3,
    is_starred: false,
    reflection: '手动管理内存确实比 GC 语言更费力，但更有掌控感。',
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["rust","ownership","string"]',
    created_at: daysAgo(20),
    updated_at: daysAgo(2),
  },

  // ---- in_progress ----
  {
    id: 'task-fe-vite',
    module_id: 'mod-fe-build',
    title: '配置 Vite 插件的二次开发',
    description: '为一个 Vite 项目编写自定义插件：自动生成路由配置。',
    difficulty: 'medium',
    estimated_minutes: 60,
    prerequisites: '了解 Vite 插件 API',
    hint_content: '使用 transform 钩子来处理模块代码。',
    reference_links: null,
    status: 'in_progress',
    started_at: daysAgo(1),
    completed_at: null,
    attempt_count: 0,
    is_starred: true,
    reflection: null,
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["vite","plugin","build"]',
    created_at: daysAgo(25),
    updated_at: daysAgo(1),
  },
  {
    id: 'task-rust-borrow',
    module_id: 'mod-rust-ownership',
    title: '实现链表数据结构',
    description: '用 Rust 实现一个单向链表，实践借用检查和生命周期标注。',
    difficulty: 'hard',
    estimated_minutes: 90,
    prerequisites: '理解借用规则',
    hint_content: '考虑使用 Option<Box<Node>> 而不是引用。',
    reference_links: '{"book":"https://rust-unofficial.github.io/too-many-lists/"}',
    status: 'in_progress',
    started_at: daysAgo(2),
    completed_at: null,
    attempt_count: 2,
    is_starred: true,
    reflection: 'Option<Box> 比裸指针安全得多。',
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["rust","linked-list","borrow"]',
    created_at: daysAgo(18),
    updated_at: daysAgo(1),
  },
  {
    id: 'task-ml-linear',
    module_id: 'mod-ml-linear',
    title: '手写梯度下降',
    description: '从零实现批量梯度下降算法，在 sklearn 的波士顿房价数据集上验证。',
    difficulty: 'medium',
    estimated_minutes: 75,
    prerequisites: '微积分基础（求导）',
    hint_content: '注意学习率的选择，可以用学习率衰减。',
    reference_links: null,
    status: 'in_progress',
    started_at: daysAgo(1),
    completed_at: null,
    attempt_count: 1,
    is_starred: false,
    reflection: null,
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["ml","gradient-descent","python"]',
    created_at: daysAgo(60),
    updated_at: daysAgo(1),
  },

  // ---- todo (status: not_started) ----
  {
    id: 'task-fe-renderprops',
    module_id: 'mod-fe-components',
    title: '用 Render Props 实现鼠标追踪器',
    description: '实现一个 MouseTracker 组件，通过 render prop 暴露鼠标位置。',
    difficulty: 'easy',
    estimated_minutes: 20,
    prerequisites: '理解 Render Props 模式',
    hint_content: null,
    reference_links: null,
    status: 'not_started',
    started_at: null,
    completed_at: null,
    attempt_count: 0,
    is_starred: false,
    reflection: null,
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["react","render-props"]',
    created_at: daysAgo(27),
    updated_at: daysAgo(27),
  },
  {
    id: 'task-fe-webpack',
    module_id: 'mod-fe-build',
    title: '编写 Webpack Loader',
    description: '编写一个 markdown-loader，将 .md 文件转换为 HTML 字符串。',
    difficulty: 'medium',
    estimated_minutes: 40,
    prerequisites: '了解 Webpack Loader API',
    hint_content: null,
    reference_links: null,
    status: 'not_started',
    started_at: null,
    completed_at: null,
    attempt_count: 0,
    is_starred: false,
    reflection: null,
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["webpack","loader","build"]',
    created_at: daysAgo(23),
    updated_at: daysAgo(23),
  },
  {
    id: 'task-rust-concurrency',
    module_id: 'mod-rust-concurrency',
    title: '实现线程池',
    description: '基于 Rust 标准库实现一个简单的线程池，支持任务提交和结果获取。',
    difficulty: 'hard',
    estimated_minutes: 120,
    prerequisites: '理解 Arc<Mutex> 和 channel',
    hint_content: null,
    reference_links: null,
    status: 'not_started',
    started_at: null,
    completed_at: null,
    attempt_count: 0,
    is_starred: false,
    reflection: null,
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["rust","concurrency","thread-pool"]',
    created_at: daysAgo(14),
    updated_at: daysAgo(14),
  },

  // ---- blocked (用地特殊 status 如 "reviewed" 来表示 blocked 场景) ----
  // 实际上 mock 用 existing statuses: not_started/in_progress/completed/reviewed
  // 用 reviewed 模拟 blocked（需要 review）
  {
    id: 'task-fe-monorepo',
    module_id: 'mod-fe-monorepo',
    title: '搭建 Turborepo 项目',
    description: '使用 Turborepo + pnpm workspace 创建一个包含两个 app 和三个 shared package 的项目。',
    difficulty: 'hard',
    estimated_minutes: 90,
    prerequisites: '了解 pnpm workspace',
    hint_content: '关注 cache 和 remote caching 配置。',
    reference_links: null,
    status: 'reviewed',
    started_at: daysAgo(10),
    completed_at: daysAgo(5),
    attempt_count: 4,
    is_starred: false,
    reflection: '远程缓存配置失败了，需要重新检查 token。',
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["turborepo","monorepo","build"]',
    created_at: daysAgo(20),
    updated_at: daysAgo(5),
  },
  {
    id: 'task-fe-micro',
    module_id: 'mod-fe-micro',
    title: 'Module Federation 集成',
    description: '基于 Webpack 5 Module Federation 将两个独立应用集成到一个容器应用。',
    difficulty: 'hard',
    estimated_minutes: 120,
    prerequisites: '了解 Webpack 5 配置',
    hint_content: '注意 shared 依赖的版本策略。',
    reference_links: null,
    status: 'reviewed',
    started_at: daysAgo(8),
    completed_at: daysAgo(3),
    attempt_count: 3,
    is_starred: true,
    reflection: '联邦模块的样式隔离还没搞定。',
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["micro-frontend","webpack","federation"]',
    created_at: daysAgo(15),
    updated_at: daysAgo(3),
  },
  {
    id: 'task-ml-linear-rev',
    module_id: 'mod-ml-linear',
    title: '正则化与过拟合分析',
    description: '在房价预测任务上对比 L1/L2 正则化的效果，分析不同正则化强度的 Bias-Variance 权衡。',
    difficulty: 'medium',
    estimated_minutes: 60,
    prerequisites: '完成梯度下降任务',
    hint_content: '使用 sklearn 的 Ridge 和 Lasso 进行交叉验证。',
    reference_links: null,
    status: 'reviewed',
    started_at: daysAgo(3),
    completed_at: daysAgo(1),
    attempt_count: 2,
    is_starred: false,
    reflection: 'L1 产生稀疏解的特性很适合特征选择。',
    code_snippets: null,
    screenshots_json: null,
    created_wiki_pages: '',
    related_wiki_pages: '',
    tags: '["ml","regularization","bias-variance"]',
    created_at: daysAgo(58),
    updated_at: daysAgo(1),
  },
];

// ===================================================================
// 4. 每日日志 (TaskDailyLog) × 7 天
// ===================================================================
export const taskDailyLogs = [
  {
    id: 'log-day-0',
    date: daysAgo(0).slice(0, 10),
    total_minutes: 90,
    tasks_completed: 1,
    tasks_in_progress: 3,
    streak_day: 5,
    reflection: '完成了 Rust 所有权任务，开始搞链表。',
    created_at: daysAgo(0),
  },
  {
    id: 'log-day-1',
    date: daysAgo(1).slice(0, 10),
    total_minutes: 120,
    tasks_completed: 0,
    tasks_in_progress: 2,
    streak_day: 4,
    reflection: 'Vite 插件开发有点卡住了，需要看文档。',
    created_at: daysAgo(1),
  },
  {
    id: 'log-day-2',
    date: daysAgo(2).slice(0, 10),
    total_minutes: 60,
    tasks_completed: 1,
    tasks_in_progress: 2,
    streak_day: 3,
    reflection: '完成了 withLogger HOC，原来 forwardRef 是这么用的。',
    created_at: daysAgo(2),
  },
  {
    id: 'log-day-3',
    date: daysAgo(3).slice(0, 10),
    total_minutes: 45,
    tasks_completed: 1,
    tasks_in_progress: 3,
    streak_day: 2,
    reflection: 'Panel 组件通过了 code review。',
    created_at: daysAgo(3),
  },
  {
    id: 'log-day-4',
    date: daysAgo(4).slice(0, 10),
    total_minutes: 30,
    tasks_completed: 0,
    tasks_in_progress: 2,
    streak_day: 1,
    reflection: '今天状态不太好，只看了会儿文档。',
    created_at: daysAgo(4),
  },
  {
    id: 'log-day-5',
    date: daysAgo(5).slice(0, 10),
    total_minutes: 75,
    tasks_completed: 1,
    tasks_in_progress: 1,
    streak_day: 0,
    reflection: '完成了第一个 Rust 任务，Rust 的所有权系统确实需要花时间理解。',
    created_at: daysAgo(5),
  },
  {
    id: 'log-day-6',
    date: daysAgo(6).slice(0, 10),
    total_minutes: 55,
    tasks_completed: 1,
    tasks_in_progress: 2,
    streak_day: 0,
    reflection: '开始了 Rust 路径的学习。',
    created_at: daysAgo(6),
  },
];

// ===================================================================
// 5. Wiki 页面 (WikiPage) × 8 — 含真实 Markdown 正文
// ===================================================================
export const wikiPages = [
  {
    id: 'wiki-comp-patterns',
    title: 'React 组件设计模式总结',
    slug: 'react-component-patterns',
    page_type: 'concept' as const,
    body_markdown: `# React 组件设计模式总结

## 一、组合模式 (Composition)

React 推荐用组合而非继承来复用代码。

### 基本用法

\`\`\`tsx
<Panel
  header={<h2>标题</h2>}
  footer={<button>确定</button>}
>
  <p>这是主体内容</p>
</Panel>
\`\`\`

### 优势
- 类型安全
- 灵活度高
- 与 React 数据流一致

## 二、高阶组件 (HOC)

> HOC 是一个函数，接受组件作为参数，返回增强后的新组件。

常见用途：权限校验、日志记录、数据预取。

## 三、Render Props

通过函数 prop 共享代码，比 HOC 更显式。

\`\`\`tsx
<MouseTracker>
  {({ x, y }) => <p>鼠标位置: {x}, {y}</p>}
</MouseTracker>
\`\`\`

## 四、Hooks 时代的选择

Hooks 能替代大部分 HOC 和 Render Props 的场景，但组合仍然是基础。

| 模式 | 推荐度 | 说明 |
|------|--------|------|
| 组合 | ⭐⭐⭐ | 基础，永远优先考虑 |
| Hooks | ⭐⭐⭐ | 逻辑复用首选 |
| HOC | ⭐⭐ | 特殊情况使用 |
| Render Props | ⭐ | 较少使用 |
`,
    summary: '系统总结 React 的四种组件设计模式及其适用场景。',
    tags: '["react","components","patterns","design"]',
    status: 'active',
    confidence: 0.92,
    created_at: daysAgo(15),
    updated_at: daysAgo(3),
    last_compiled_at: daysAgo(3),
    source_message_id: null,
    author_name: 'OpenWiki 自动生成',
    author_url: null,
    source_type: 'reflection',
  },
  {
    id: 'wiki-rust-ownership',
    title: 'Rust 所有权系统详解',
    slug: 'rust-ownership-system',
    page_type: 'concept' as const,
    body_markdown: `# Rust 所有权系统详解

## 三条核心规则

1. **每个值有且只有一个所有者**
2. **当所有者离开作用域，值被自动释放**
3. **所有权可以转移（Move）或借用（Borrow）**

### Move 语义

\`\`\`rust
let s1 = String::from("hello");
let s2 = s1; // s1 失效，所有权转移到 s2
\`\`\`

### Borrow 规则

- 任意时刻只能有 **一个可变引用** 或 **多个不可变引用**
- 引用必须始终有效（无悬垂引用）

### 生命周期

\`\`\`rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
\`\`\`

> 生命周期标注不会改变引用的实际存活时间，只告诉编译器做检查。

## 为什么需要所有权？

- 无 GC 下的内存安全
- 无数据竞争
- 零成本抽象
`,
    summary: '全面的 Rust 所有权指南，包括 Move、Borrow 和生命周期。',
    tags: '["rust","ownership","borrow","lifetime"]',
    status: 'active',
    confidence: 0.88,
    created_at: daysAgo(10),
    updated_at: daysAgo(2),
    last_compiled_at: daysAgo(2),
    source_message_id: null,
    author_name: null,
    author_url: null,
    source_type: null,
  },
  {
    id: 'wiki-vite-plugin',
    title: 'Vite 插件开发快速入门',
    slug: 'vite-plugin-development',
    page_type: 'how-to' as const,
    body_markdown: `# Vite 插件开发快速入门

## 插件结构

一个 Vite 插件就是一个对象，包含名称和钩子函数。

\`\`\`ts
function myPlugin(): Plugin {
  return {
    name: 'my-plugin',
    resolveId(id) {
      if (id === 'virtual-module') return id;
    },
    load(id) {
      if (id === 'virtual-module') return 'export default "virtual"';
    },
    transform(code, id) {
      if (id.endsWith('.custom')) {
        return code.replace(/\\bvar\\b/g, 'let');
      }
    },
  };
}
\`\`\`

## 常用钩子

| 钩子 | 时机 | 用途 |
|------|------|------|
| resolveId | 模块解析 | 别名、虚拟模块 |
| load | 模块加载 | 提供自定义内容 |
| transform | 代码转换 | 转译、注入代码 |
| config | 配置解析 | 修改 Vite 配置 |

## 自动路由插件示例

\`\`\`ts
function AutoRoutesPlugin(dir: string): Plugin {
  return {
    name: 'auto-routes',
    transform(code, id) {
      if (id.includes(dir)) {
        // 读取文件系统生成路由
      }
    },
  };
}
\`\`\`
`,
    summary: '一步步教你编写 Vite 插件，附带自动路由生成实战示例。',
    tags: '["vite","plugin","build","tooling"]',
    status: 'active',
    confidence: 0.85,
    created_at: daysAgo(8),
    updated_at: daysAgo(1),
    last_compiled_at: daysAgo(1),
    source_message_id: null,
    author_name: null,
    author_url: null,
    source_type: null,
  },
  {
    id: 'wiki-gradient-descent',
    title: '梯度下降算法详解',
    slug: 'gradient-descent-algorithm',
    page_type: 'concept' as const,
    body_markdown: `# 梯度下降算法详解

## 核心思想

沿着损失函数的梯度反方向更新参数，逐步逼近最优解。

### 公式

\\[
\\theta_{t+1} = \\theta_t - \\eta \\cdot \\nabla_\\theta J(\\theta_t)
\\]

其中 \\(\\eta\\) 是学习率，\\(\\nabla_\\theta J\\) 是梯度。

## 三种变体

### 1. 批量梯度下降 (BGD)

每次使用全部样本计算梯度。

**优点**：方向准确  
**缺点**：大数据集缓慢

### 2. 随机梯度下降 (SGD)

每次随机选一个样本。

**优点**：快速，可跳出局部最优  
**缺点**：震荡大

### 3. Mini-Batch 梯度下降

**最常用**：每次用一小批样本。

## 学习率策略

- 固定学习率
- 学习率衰减（Step Decay / Exponential Decay）
- 自适应学习率（Adam, RMSprop）

## Python 实现

\`\`\`python
import numpy as np

def gradient_descent(X, y, lr=0.01, epochs=1000):
    m, n = X.shape
    theta = np.zeros(n)
    for epoch in range(epochs):
        gradient = (1/m) * X.T @ (X @ theta - y)
        theta -= lr * gradient
    return theta
\`\`\`
`,
    summary: '梯度下降的三种变体、学习率策略及纯 Python 实现。',
    tags: '["ml","gradient-descent","optimization"]',
    status: 'active',
    confidence: 0.9,
    created_at: daysAgo(12),
    updated_at: daysAgo(1),
    last_compiled_at: null,
    source_message_id: null,
    author_name: null,
    author_url: null,
    source_type: null,
  },
  {
    id: 'wiki-regularization',
    title: 'L1/L2 正则化对比',
    slug: 'l1-l2-regularization',
    page_type: 'comparison' as const,
    body_markdown: `# L1 与 L2 正则化对比

## 为什么需要正则化？

防止过拟合，提高模型泛化能力。

## L1 正则化（Lasso）

\\[
J(\\theta) = MSE(\\theta) + \\lambda \\sum |\\theta_i|
\\]

**特点**：产生稀疏解，自动做特征选择。

## L2 正则化（Ridge）

\\[
J(\\theta) = MSE(\\theta) + \\lambda \\sum \\theta_i^2
\\]

**特点**：权重趋向均匀分布，没有稀疏性。

## 对比表

| 特性 | L1 (Lasso) | L2 (Ridge) |
|------|------------|------------|
| 解的形式 | 稀疏 | 非稀疏 |
| 特征选择 | ✅ 自动选择 | ❌ 保留所有 |
| 可解释性 | ✅ 更好 | ❌ 较差 |
| 计算稳定性 | ⚠️ 可能不稳定 | ✅ 稳定 |
| 适用场景 | 高维特征选择 | 一般正则化 |

## Elastic Net

结合 L1 和 L2：\\(J = MSE + \\lambda_1 \\sum|\\theta| + \\lambda_2 \\sum\\theta^2\\)
`,
    summary: 'L1 (Lasso) vs L2 (Ridge) 正则化的数学原理和实际效果对比。',
    tags: '["ml","regularization","l1","l2","ridge","lasso"]',
    status: 'active',
    confidence: 0.87,
    created_at: daysAgo(7),
    updated_at: daysAgo(0),
    last_compiled_at: null,
    source_message_id: null,
    author_name: null,
    author_url: null,
    source_type: null,
  },
  {
    id: 'wiki-webpack-loader',
    title: 'Webpack Loader 编写指南',
    slug: 'webpack-loader-guide',
    page_type: 'how-to' as const,
    body_markdown: `# Webpack Loader 编写指南

## Loader 是什么？

Loader 是 Webpack 中用于转换模块源文件的函数。

\`\`\`js
// 最简单的 loader
module.exports = function(source) {
  return source.replace(/\\bdebugger\\b/g, '');
};
\`\`\`

## Loader 特性

- **链式调用**：从右向左执行
- **同步/异步**：可以返回 callback
- **可配置**：通过 options 接收参数

## markdown-loader 示例

\`\`\`js
const marked = require('marked');

module.exports = function(source) {
  const html = marked.parse(source);
  return \`module.exports = \${JSON.stringify(html)}\`;
};
\`\`\`

## 常用 API

| API | 说明 |
|-----|------|
| this.context | 当前文件所在目录 |
| this.resourcePath | 当前处理文件的路径 |
| this.async() | 转为异步模式 |
| this.emitFile() | 输出文件 |
`,
    summary: 'Webpack Loader 的核心概念与实战教程。',
    tags: '["webpack","loader","build-tool"]',
    status: 'active',
    confidence: 0.82,
    created_at: daysAgo(5),
    updated_at: daysAgo(0),
    last_compiled_at: null,
    source_message_id: null,
    author_name: null,
    author_url: null,
    source_type: null,
  },
  {
    id: 'wiki-turborepo-setup',
    title: 'Turborepo + pnpm Monorepo 搭建指南',
    slug: 'turborepo-pnpm-monorepo',
    page_type: 'how-to' as const,
    body_markdown: `# Turborepo + pnpm Monorepo 搭建指南

## 初始化

\`\`\`bash
npx create-turbo@latest my-monorepo
cd my-monorepo
pnpm install
\`\`\`

## 目录结构

\`\`\`
apps/
  web/       # Next.js 应用
  docs/      # 文档站点
packages/
  ui/        # 共享 UI 组件
  shared/    # 共享工具函数
  config/    # ESLint/TS 配置
\`\`\`

## 关键配置

### turbo.json

\`\`\`json
{
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", ".next/**"]
    },
    "test": {},
    "lint": {}
  }
}
\`\`\`

## Remote Caching

> 利用 Vercel Remote Cache，团队共享构建缓存。

\`\`\`bash
npx turbo login
npx turbo link
\`\`\`
`,
    summary: '从零搭建 Turborepo + pnpm 的 Monorepo 工程。',
    tags: '["turborepo","monorepo","pnpm","build"]',
    status: 'active',
    confidence: 0.75,
    created_at: daysAgo(3),
    updated_at: daysAgo(0),
    last_compiled_at: null,
    source_message_id: null,
    author_name: null,
    author_url: null,
    source_type: null,
  },
  {
    id: 'wiki-micro-frontend',
    title: '微前端架构设计参考',
    slug: 'micro-frontend-architecture',
    page_type: 'reference' as const,
    body_markdown: `# 微前端架构设计参考

## 什么是微前端？

> 微前端是一种将前端应用拆分为多个独立开发、独立部署的小型应用的架构模式。

## 主流方案对比

| 方案 | 技术 | 隔离性 | 通信 | 适用场景 |
|------|------|--------|------|----------|
| Module Federation | Webpack 5 | 中 | 运行时共享 | 大型中后台 |
| iframe | 原生 | 强 | postMessage | 完全隔离子应用 |
| qiankun | single-spa | 中 | 沙箱 + 通信 | 存量应用拆分 |
| Micro-app | Web Components | 强 | 自定义事件 | 新项目 |

## Module Federation

### 配置方式

\`\`\`js
new ModuleFederationPlugin({
  name: 'container',
  remotes: {
    app1: 'app1@http://localhost:3001/remoteEntry.js',
  },
  shared: {
    react: { singleton: true },
    'react-dom': { singleton: true },
  },
})
\`\`\`

## 注意事项

1. **样式隔离**：CSS Modules / CSS-in-JS / Shadow DOM
2. **状态共享**：通过 shared 库或自定义事件
3. **路由分配**：统一的路由管理策略
`,
    summary: '微前端架构各方案对比，重点介绍 Module Federation。',
    tags: '["micro-frontend","architecture","federation","webpack"]',
    status: 'active',
    confidence: 0.8,
    created_at: daysAgo(2),
    updated_at: daysAgo(0),
    last_compiled_at: null,
    source_message_id: null,
    author_name: null,
    author_url: null,
    source_type: null,
  },
];

// ===================================================================
// 6. Wiki 页面链接 (WikiEdge) × 3
// ===================================================================
export const wikiEdges = [
  {
    id: 1,
    source_page_id: 'wiki-comp-patterns',
    target_page_id: 'wiki-vite-plugin',
    relation: 'extends' as const,
    weight: 0.6,
    created_at: daysAgo(3),
  },
  {
    id: 2,
    source_page_id: 'wiki-gradient-descent',
    target_page_id: 'wiki-regularization',
    relation: 'extends' as const,
    weight: 0.8,
    created_at: daysAgo(2),
  },
  {
    id: 3,
    source_page_id: 'wiki-webpack-loader',
    target_page_id: 'wiki-micro-frontend',
    relation: 'related' as const,
    weight: 0.4,
    created_at: daysAgo(1),
  },
];

// ===================================================================
// 7. 待复习项 (ReviewSchedule) × 5 — 不同间隔天数
// ===================================================================
export const reviewSchedules = [
  {
    id: 'review-comp-patterns',
    wiki_page_id: 'wiki-comp-patterns',
    ease_factor: 2.5,
    interval_days: 1,
    next_review_at: daysLater(0), // 今天到期
    review_count: 1,
    last_reviewed_at: daysAgo(1),
    mastery: 0.65,
    is_archived: false,
    variant_streak: 0,
    variant_mode: 0,
    created_at: daysAgo(3),
    updated_at: daysAgo(1),
  },
  {
    id: 'review-rust-ownership',
    wiki_page_id: 'wiki-rust-ownership',
    ease_factor: 2.2,
    interval_days: 3,
    next_review_at: daysLater(1), // 明天到期
    review_count: 2,
    last_reviewed_at: daysAgo(3),
    mastery: 0.72,
    is_archived: false,
    variant_streak: 1,
    variant_mode: 2,
    created_at: daysAgo(8),
    updated_at: daysAgo(3),
  },
  {
    id: 'review-gradient-descent',
    wiki_page_id: 'wiki-gradient-descent',
    ease_factor: 1.8,
    interval_days: 7,
    next_review_at: daysLater(4), // 4天后到期
    review_count: 3,
    last_reviewed_at: daysAgo(7),
    mastery: 0.88,
    is_archived: false,
    variant_streak: 2,
    variant_mode: 4,
    created_at: daysAgo(21),
    updated_at: daysAgo(7),
  },
  {
    id: 'review-vite-plugin',
    wiki_page_id: 'wiki-vite-plugin',
    ease_factor: 2.5,
    interval_days: 1,
    next_review_at: daysLater(0),
    review_count: 1,
    last_reviewed_at: daysAgo(1),
    mastery: 0.58,
    is_archived: false,
    variant_streak: 0,
    variant_mode: 0,
    created_at: daysAgo(2),
    updated_at: daysAgo(1),
  },
  {
    id: 'review-webpack-loader',
    wiki_page_id: 'wiki-webpack-loader',
    ease_factor: 3.0,
    interval_days: 14,
    next_review_at: daysLater(12),
    review_count: 5,
    last_reviewed_at: daysAgo(14),
    mastery: 0.95,
    is_archived: false,
    variant_streak: 4,
    variant_mode: 5,
    created_at: daysAgo(60),
    updated_at: daysAgo(14),
  },
];

// ===================================================================
// 8. 待发现内容 (PendingContent) × 4
// ===================================================================
export const pendingContent = [
  {
    id: 'pending-rust-blog',
    title: 'Rust 2026 路线图：异步生态的下一步',
    source_url: 'https://blog.rust-lang.org/2026/roadmap',
    source_name: 'Rust 官方博客',
    content_summary: 'Rust 团队发布了 2026 年路线图，重点推进异步生态、编译速度和 IDE 体验。',
    source_page_id: 'wiki-rust-ownership',
    source_page_title: 'Rust 所有权系统详解',
    match_reason: '关键词匹配: Rust, 异步',
    match_keywords: 'rust,async,roadmap',
    relevance_score: 0.85,
    full_content: null,
    content_hash: 'abc123def',
    status: 'unread',
    read_at: null,
    imported_content_id: null,
    discovered_at: daysAgo(0),
    created_at: daysAgo(0),
  },
  {
    id: 'pending-micro-zach',
    title: 'Zach Leatherman: 微前端是不是过度工程？',
    source_url: 'https://www.zachleat.com/web/micro-frontends/',
    source_name: 'Zach Leatherman',
    content_summary: '一位资深前端开发者对微前端架构的反思，认为大部分场景不需要微前端。',
    source_page_id: 'wiki-micro-frontend',
    source_page_title: '微前端架构设计参考',
    match_reason: '标签匹配: micro-frontend',
    match_keywords: 'micro-frontend,architecture',
    relevance_score: 0.72,
    full_content: null,
    content_hash: 'def456ghi',
    status: 'reading',
    read_at: daysAgo(0),
    imported_content_id: null,
    discovered_at: daysAgo(1),
    created_at: daysAgo(1),
  },
  {
    id: 'pending-ml-news',
    title: 'DeepMind 发布新优化器：比 Adam 快 2 倍',
    source_url: 'https://deepmind.google/research/optimizer',
    source_name: 'Google DeepMind',
    content_summary: 'DeepMind 团队提出一种新的自适应优化器，在多项基准测试上超越 Adam 和 AdamW。',
    source_page_id: 'wiki-gradient-descent',
    source_page_title: '梯度下降算法详解',
    match_reason: '话题匹配: 优化器, 梯度下降',
    match_keywords: 'optimizer,gradient-descent,adam',
    relevance_score: 0.78,
    full_content: null,
    content_hash: 'ghi789jkl',
    status: 'imported',
    read_at: daysAgo(1),
    imported_content_id: 'content-deepmind-opt',
    discovered_at: daysAgo(2),
    created_at: daysAgo(2),
  },
  {
    id: 'pending-react-clip',
    title: 'React Server Components 实战经验分享',
    source_url: 'https://example.com/rsc-experience',
    source_name: '剪藏',
    content_summary: '一线团队使用 RSC 半年后的经验总结，包含性能数据和陷阱。',
    source_page_id: null,
    source_page_title: null,
    match_reason: null,
    match_keywords: 'react,server-components,rsc',
    relevance_score: 0.6,
    full_content: '这是一篇从网页剪藏下来的长文...（略）',
    content_hash: 'jkl012mno',
    status: 'dismissed',
    read_at: daysAgo(3),
    imported_content_id: null,
    discovered_at: daysAgo(3),
    created_at: daysAgo(3),
  },
];

// ===================================================================
// 9. 监视器配置 (MonitorSource) × 2
// ===================================================================
export const monitorSources = [
  {
    id: 'monitor-rust-blog',
    page_id: 'wiki-rust-ownership',
    search_query: 'Rust 最新动态',
    source_type: 'rss',
    rss_url: 'https://blog.rust-lang.org/feed.xml',
    is_active: true,
    last_checked_at: daysAgo(0),
    last_found_count: 3,
  },
  {
    id: 'monitor-ml-arxiv',
    page_id: 'wiki-gradient-descent',
    search_query: '机器学习优化算法',
    source_type: 'rss',
    rss_url: 'https://arxiv.org/rss/cs.LG',
    is_active: true,
    last_checked_at: daysAgo(1),
    last_found_count: 7,
  },
];

// ===================================================================
// 10. 知识排行 (KnowledgeRanking)
// ===================================================================
export const knowledgeRanking = {
  total_score: 2840,
  level: '白银学者',
  level_color: '#94A3B8',
  breadth: { score: 720, max_score: 1000, percentage: 72, label: '广度', icon: 'globe' },
  depth: { score: 680, max_score: 1000, percentage: 68, label: '深度', icon: 'layers' },
  mastery: { score: 590, max_score: 1000, percentage: 59, label: '掌握度', icon: 'brain' },
  discovery: { score: 450, max_score: 1000, percentage: 45, label: '发现', icon: 'compass' },
  connections: { score: 400, max_score: 1000, percentage: 40, label: '关联', icon: 'share2' },
  tag_distribution: [
    { tag: 'React', page_count: 3, avg_mastery: 0.82 },
    { tag: 'Rust', page_count: 1, avg_mastery: 0.72 },
    { tag: '机器学习', page_count: 2, avg_mastery: 0.88 },
    { tag: '构建工具', page_count: 2, avg_mastery: 0.75 },
  ],
};

// ===================================================================
// 11. 学习排行 (LearningRanking)
// ===================================================================
export const learningRanking = {
  total_score: 1560,
  level: '青铜学者',
  level_color: '#CD7F32',
  consistency: { score: 420, max_score: 1000, percentage: 42, label: '连续性', icon: 'calendar' },
  completion: { score: 380, max_score: 1000, percentage: 38, label: '完成率', icon: 'check-circle' },
  quality: { score: 450, max_score: 1000, percentage: 45, label: '质量', icon: 'star' },
  dedication: { score: 310, max_score: 1000, percentage: 31, label: '投入度', icon: 'clock' },
  stats_summary: {
    streak_day: 5,
    completed_tasks: 5,
    total_tasks: 12,
    total_reviews: 8,
    avg_quality: 3.2,
    total_minutes: 475,
  },
};

// ===================================================================
// 12. 默认设置 (Default Settings)
// ===================================================================
export const defaultSettings: Record<string, string> = {
  api_key: '',
  provider: 'anthropic',
  model: 'claude-sonnet-4-6',
  custom_base_url: '',
  theme: 'light',
  language_mode: 'cn',
  capture_enabled: 'true',
  capture_mode: 'region',
  bubble_style: 'adaptive',
  bubble_position: 'bottom-right',
  default_action: 'popover',
  sensitive_filter_enabled: 'true',
  url_reading_enabled: 'true',
  radar_interval_days: '7',
  countdown_duration: '5',
  screenshot_dir: '~/Pictures/OpenWiki',
  oauth_logged_in: 'false',
  oauth_email: '',
  gemini_oauth_logged_in: 'false',
  gemini_oauth_email: '',
};

// ===================================================================
// 13. 仪表盘统计数据
// ===================================================================
export const dashboardStats = {
  active_paths: 2,
  total_tasks: 12,
  completed_tasks: 3,
  in_progress_tasks: 3,
  pending_tasks: 3,
  streak_day: 5,
  total_study_minutes: 475,
  weekly_activity: [
    { day: '周一', minutes: 90 },
    { day: '周二', minutes: 120 },
    { day: '周三', minutes: 60 },
    { day: '周四', minutes: 45 },
    { day: '周五', minutes: 30 },
    { day: '周六', minutes: 75 },
    { day: '周日', minutes: 55 },
  ],
  wiki_page_count: 8,
  wiki_edge_count: 3,
  due_reviews: 2,
  unread_discoveries: 1,
};

// ===================================================================
// 14. 统一导出
// ===================================================================
const mockData = {
  learningPaths,
  modules,
  practiceTasks,
  taskDailyLogs,
  wikiPages,
  wikiEdges,
  reviewSchedules,
  pendingContent,
  monitorSources,
  knowledgeRanking,
  learningRanking,
  defaultSettings,
  dashboardStats,
  daysAgo,
  daysLater,
  today,
};

export default mockData;
