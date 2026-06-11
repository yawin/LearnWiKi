// =====================================================================
// OpenWiki Mock Command Router
// 125+ invoke 命令的路由表
// =====================================================================

import mockData from './data/all';

// ===================================================================
// 类型定义
// ===================================================================
export type CommandHandler = (args?: Record<string, unknown>) => unknown;

// ===================================================================
// 核心命令 (~25 个) — 返回真实 mock 数据
// ===================================================================
const coreHandlers: Record<string, CommandHandler> = {
  // ----- 学习路径 -----
  get_learning_paths: () => mockData.learningPaths,
  get_learning_path: (args) =>
    mockData.learningPaths.find((p) => p.id === args?.id) ?? null,
  create_learning_path: (_args) => ({
    id: `path-mock-${Date.now()}`,
    title: _args?.title ?? '',
    description: _args?.description ?? '',
    topic: _args?.topic ?? '',
    difficulty: _args?.difficulty ?? 'beginner',
    estimated_days: (_args?.estimatedDays as number) ?? 30,
    module_count: 0,
    completion_rate: 0,
    is_active: true,
    created_at: mockData.today(),
    updated_at: mockData.today(),
  }),
  update_learning_path: (args) => {
    const existing = mockData.learningPaths.find((p) => p.id === args?.id);
    return existing
      ? { ...existing, ...args, updated_at: mockData.today() }
      : null;
  },
  delete_learning_path: () => ({ success: true }),

  // ----- 模块 -----
  get_modules_by_path: (args) =>
    mockData.modules.filter((m) => m.path_id === args?.pathId),
  create_module: (_args) => ({
    id: `mod-mock-${Date.now()}`,
    path_id: _args?.pathId as string,
    title: (_args?.title as string) ?? '',
    sort_order: (_args?.sortOrder as number) ?? 0,
    description: (_args?.description as string) ?? '',
    theory_markdown: '',
    reading_list_json: '[]',
    estimated_read_minutes: (_args?.estimatedReadMinutes as number) ?? 30,
    discussion_prompts: '',
    community_solutions: '',
    task_ids: '',
    status: 'available',
    completed_at: null,
    created_at: mockData.today(),
    updated_at: mockData.today(),
  }),
  update_module: (args) => {
    const existing = mockData.modules.find((m) => m.id === args?.id);
    return existing
      ? { ...existing, ...args, updated_at: mockData.today() }
      : null;
  },
  delete_module: () => ({ success: true }),

  // ----- 任务 -----
  get_tasks_by_module: (args) =>
    mockData.practiceTasks.filter((t) => t.module_id === args?.moduleId),
  get_task_by_id: (args) =>
    mockData.practiceTasks.find((t) => t.id === args?.id) ?? null,
  get_all_tasks: () => mockData.practiceTasks,
  create_practice_task: (_args) => ({
    id: `task-mock-${Date.now()}`,
    module_id: (_args?.moduleId as string) ?? '',
    title: (_args?.title as string) ?? '',
    description: (_args?.description as string) ?? '',
    difficulty: (_args?.difficulty as string) ?? 'easy',
    estimated_minutes: (_args?.estimatedMinutes as number) ?? 30,
    prerequisites: (_args?.prerequisites as string) ?? '',
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
    tags: null,
    created_at: mockData.today(),
    updated_at: mockData.today(),
  }),
  update_task_status: (args) => {
    const task = mockData.practiceTasks.find((t) => t.id === args?.id);
    if (!task) return { success: false, error: 'Task not found' };
    return { ...task, status: args?.status as string, updated_at: mockData.today() };
  },
  update_practice_task: (args) => {
    const task = mockData.practiceTasks.find((t) => t.id === args?.id);
    if (!task) return { success: false, error: 'Task not found' };
    return { ...task, ...args, updated_at: mockData.today() };
  },
  delete_practice_task: () => ({ success: true }),

  // ----- 日志 -----
  get_recent_logs: () => mockData.taskDailyLogs,
  record_daily_log: (_args) => ({
    id: `log-mock-${Date.now()}`,
    date: (_args?.date as string) ?? mockData.today().slice(0, 10),
    total_minutes: (_args?.totalMinutes as number) ?? 0,
    tasks_completed: (_args?.tasksCompleted as number) ?? 0,
    tasks_in_progress: (_args?.tasksInProgress as number) ?? 0,
    streak_day: (_args?.streakDay as number) ?? 0,
    reflection: (_args?.reflection as string) ?? null,
    created_at: mockData.today(),
  }),

  // ----- 学习统计 -----
  get_learning_stats: () => ({
    streak_day: 5,
    completion_rate: 0.25,
    total_minutes: 475,
    total_tasks: 12,
    completed_tasks: 3,
    weekly_data: mockData.taskDailyLogs,
    topic_distribution: [
      { topic: '前端开发', count: 5 },
      { topic: '系统编程', count: 4 },
      { topic: '人工智能', count: 3 },
    ],
  }),
  update_study_time: (args) => ({
    id: `log-${mockData.today().slice(0, 10)}`,
    date: mockData.today().slice(0, 10),
    total_minutes: (args?.additionalMinutes as number) ?? 0,
    tasks_completed: 0,
    tasks_in_progress: 0,
    streak_day: 5,
    reflection: null,
    created_at: mockData.today(),
  }),

  // ----- Wiki 页面 -----
  get_wiki_pages: (_args) => mockData.wikiPages,
  get_wiki_page: (args) =>
    mockData.wikiPages.find((p) => p.id === args?.id) ?? null,
  search_wiki: (args) => {
    const q = ((args?.query as string) ?? '').toLowerCase();
    return mockData.wikiPages.filter(
      (p) =>
        p.title.toLowerCase().includes(q) ||
        p.tags?.toLowerCase().includes(q)
    );
  },
  get_wiki_stats: () => ({
    total_pages: mockData.wikiPages.length,
    total_edges: mockData.wikiEdges.length,
    total_sources: 0,
    needs_recompile: 0,
    lint_open: 0,
  }),
  delete_wiki_page: () => ({ success: true }),
  get_wiki_graph: () => ({
    nodes: mockData.wikiPages.map((p) => ({
      id: p.id,
      title: p.title,
      page_type: p.page_type,
      status: p.status,
      confidence: p.confidence,
      edge_count: mockData.wikiEdges.filter(
        (e) => e.source_page_id === p.id || e.target_page_id === p.id
      ).length,
    })),
    edges: mockData.wikiEdges.map((e) => ({
      source: e.source_page_id,
      target: e.target_page_id,
      relation: e.relation,
      weight: e.weight,
    })),
  }),

  // ----- Wiki 知识连接 -----
  get_wiki_pages_for_task: (args) => {
    const task = mockData.practiceTasks.find((t) => t.id === args?.taskId);
    if (!task) return [];
    const wikiIds = task.related_wiki_pages
      ? task.related_wiki_pages.split(',').filter(Boolean)
      : [];
    return mockData.wikiPages
      .filter((p) => wikiIds.includes(p.id))
      .map((p) => ({
        ...p,
        author_name: p.author_name ?? null,
        author_url: p.author_url ?? null,
        source_type: p.source_type ?? null,
      }));
  },
  get_tasks_for_wiki: (args) =>
    mockData.practiceTasks.filter((t) =>
      t.related_wiki_pages.includes(args?.wikiId as string)
    ),
  auto_match_wiki_pages: () => [],
  link_task_to_wiki: () => ({ success: true }),
  unlink_task_from_wiki: () => ({ success: true }),

  // ----- 任务解决方案 -----
  get_solutions_for_task: () => [],
  create_task_solution: () => ({ success: true }),
  update_task_solution: () => ({ success: true }),
  delete_task_solution: () => ({ success: true }),
  update_wiki_page_author: () => ({ success: true }),

  // ----- 任务推荐 -----
  get_recommendations: () => [],
  generate_recommendations: () => [],
  ignore_recommendation: () => ({ success: true }),
  accept_recommendation: (_args) => ({
    id: `task-rec-${Date.now()}`,
    module_id: '',
    title: (_args?.title as string) ?? '推荐任务',
    description: '',
    difficulty: 'medium',
    estimated_minutes: 30,
    prerequisites: '',
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
    tags: null,
    created_at: mockData.today(),
    updated_at: mockData.today(),
  }),
  dismiss_recommendation: () => ({ success: true }),

  // ----- 反思生成 Wiki -----
  generate_wiki_from_reflection: () => [],

  // ----- 复习系统 -----
  get_due_reviews: () =>
    mockData.reviewSchedules
      .filter((r) => new Date(r.next_review_at) <= new Date())
      .map((r) => {
        const wiki = mockData.wikiPages.find(
          (p) => p.id === r.wiki_page_id
        );
        return {
          schedule: r,
          wiki_title: wiki?.title ?? '',
          wiki_summary: wiki?.summary ?? null,
          wiki_tags: wiki?.tags ?? null,
        };
      }),
  create_review_schedule: (args) => ({
    id: `review-mock-${Date.now()}`,
    wiki_page_id: args?.wikiPageId as string,
    ease_factor: 2.5,
    interval_days: 1,
    next_review_at: mockData.today(),
    review_count: 0,
    last_reviewed_at: null,
    mastery: 0,
    is_archived: false,
    variant_streak: 0,
    variant_mode: 0,
    created_at: mockData.today(),
    updated_at: mockData.today(),
  }),
  auto_create_review_schedule: (args) => ({
    id: `review-auto-${Date.now()}`,
    wiki_page_id: args?.wikiPageId as string,
    ease_factor: 2.5,
    interval_days: 1,
    next_review_at: mockData.today(),
    review_count: 0,
    last_reviewed_at: null,
    mastery: 0,
    is_archived: false,
    variant_streak: 0,
    variant_mode: 0,
    created_at: mockData.today(),
    updated_at: mockData.today(),
  }),
  submit_review_feedback: (_args) => ({
    id: `review-${Date.now()}`,
    wiki_page_id: '',
    ease_factor: 2.5,
    interval_days: 1,
    next_review_at: mockData.daysLater(1),
    review_count: 1,
    last_reviewed_at: mockData.today(),
    mastery: 0.5,
    is_archived: false,
    variant_streak: 0,
    variant_mode: 0,
    created_at: mockData.today(),
    updated_at: mockData.today(),
  }),
  get_review_stats: () => ({
    total_due: 2,
    total_reviewed_today: 1,
    streak: 5,
  }),
  get_health_schedules: () =>
    mockData.reviewSchedules.map((r) => {
      const wiki = mockData.wikiPages.find((p) => p.id === r.wiki_page_id);
      return {
        schedule: r,
        wiki_title: wiki?.title ?? '',
        wiki_summary: wiki?.summary ?? null,
        tags: wiki?.tags ?? null,
      };
    }),
  get_review_health_stats: () => ({
    total_pages: 8,
    pages_with_reviews: 5,
    total_reviews_all_time: 12,
    avg_accuracy: 3.2,
    streak_day: 5,
    weekly_review_count: 4,
    overdue_count: 1,
    mastered_count: 2,
  }),
  get_wiki_learning_trail: (args) => ({
    schedule:
      mockData.reviewSchedules.find(
        (r) => r.wiki_page_id === args?.wikiPageId
      ) ?? null,
    recent_logs: [],
    referenced_tasks: [],
    is_due: false,
  }),

  // ----- 复习格式生成 -----
  generate_quiz_questions: () => [],
  generate_ordering_steps: () => ({ title: '', steps: [] }),
  get_available_formats: () => ['quiz', 'rapid_fire', 'ordering', 'error_hunt', 'cloze', 'explain'],
  generate_error_hunt: () => ({
    title: '模拟错误排查题',
    content: '这是一段包含错误的示例代码...',
    error_index: 2,
    options: ['选项A', '选项B', '选项C', '选项D'],
    explanation: 'C 选项中的语法错误导致编译失败。',
    correct_version: '修复后的正确版本示例。',
  }),
  generate_cloze: () => ({
    template: 'React 中的 ___ 钩子用于管理副作用。',
    blanks: [{ index: 0, correct_answers: ['useEffect', 'useEffect()'] }],
  }),
  generate_explain_review: (args) => {
    const wiki = mockData.wikiPages.find((p) => p.id === args?.wikiPageId);
    return {
      wiki_title: wiki?.title ?? '',
      wiki_summary: wiki?.summary ?? '',
      wiki_tags: wiki?.tags?.split(',') ?? [],
      prompt: `请用自己的话解释「${wiki?.title ?? '主题'}」的核心概念。`,
    };
  },
  submit_explain_answer: () => ({
    score: 75,
    score_label: '良好',
    improvement_suggestions: ['可以更具体一些', '缺少实际例子'],
    better_example: null,
    strength_points: ['结构清晰', '关键点准确'],
    weakness_points: ['描述偏笼统'],
  }),
  generate_variant_question: () => ({
    format: 'quiz',
    question_data: { stem: '变体问题？', options: ['A', 'B', 'C', 'D'], correct_index: 0, explanation: '' },
    variant_generation: 2,
    twist_description: '变换角度提问',
  }),

  // ----- 排行 -----
  get_knowledge_ranking: () => mockData.knowledgeRanking,
  get_learning_ranking: () => mockData.learningRanking,

  // ----- 设置 -----
  get_settings: () => mockData.defaultSettings,
  update_setting: () => ({ success: true }),
  check_xreader_status: () => ({
    installed: true,
    supported_platforms: ['macos', 'linux'],
    install_command: 'brew install xreader',
  }),

  // ----- 发现系统 -----
  get_pending_content: (_args) => mockData.pendingContent,
  run_discovery_for_page: () => ({ success: true }),
  get_monitor_sources_for_page: (args) =>
    mockData.monitorSources.filter((m) => m.page_id === args?.pageId),
  create_monitor_source: (_args) => ({
    id: `monitor-mock-${Date.now()}`,
    page_id: (_args?.pageId as string) ?? '',
    search_query: (_args?.searchQuery as string) ?? '',
    source_type: (_args?.sourceType as string) ?? 'rss',
    rss_url: (_args?.rssUrl as string) ?? null,
    is_active: true,
    last_checked_at: null,
    last_found_count: 0,
  }),
  update_monitor_source: () => ({ success: true }),
  test_rss_feed: () => [
    { title: 'Mock RSS 文章 1', url: 'https://example.com/1' },
    { title: 'Mock RSS 文章 2', url: 'https://example.com/2' },
  ],
  trigger_attention_analysis: () => ({ success: true }),

  // ----- 数据导出 / 备份 -----
  export_backup: () => '/tmp/openwiki-backup.zip',
  import_backup: () => ({ success: true }),
  auto_backup: () => ({ success: true }),
  export_day_markdown: () => '# Mock Export Content',
  export_all_markdown: () => 1,
  export_date_range_markdown: () => 1,
  export_all_single: () => '/tmp/openwiki-export.md',
  export_range_single: () => '/tmp/openwiki-export-range.md',
  get_export_dir: () => '~/Pictures/OpenWiki',
  set_export_dir: () => ({ success: true }),
  open_export_dir: () => ({ success: true }),
  get_dates_with_content: () => [['2026-06-01', 3], ['2026-06-02', 1]],
  get_content_for_date: () => [],
  search_content: () => [],

  // ----- 存储管理 -----
  get_all_content: () => [],
  get_storage_info: () => ({ total_items: 0, disk_usage_mb: 0 }),
  delete_content: () => ({ success: true }),
  retry_url_fetch: () => ({ success: true }),
  ocr_image: () => 'Mock OCR text',
  get_contents_by_ids: () => [],
  import_markdown_files: () => ({ success: true, count: 0 }),
  import_content_files: () => ({ success: true, count: 0 }),
  save_spotlight_content: () => ({ success: true }),

  // ----- Wiki 聊天 / 问答 -----
  wiki_ask: () => ({
    message_id: `msg-${Date.now()}`,
    answer: '这是一个模拟的问答回复。',
    pages_used: [],
    source_mode: 'knowledge_base',
    confidence: 0.85,
  }),
  get_chat_sessions: () => [],
  get_chat_messages: () => [],
  delete_chat_session: () => ({ success: true }),
  save_message_as_page: () => null,
  get_saved_message_ids: () => [],
  get_wiki_conversations: () => [],

  // ----- Wiki 标签连接 / Lint -----
  wiki_link_by_tags: () => ({ edges_created: 0 }),
  trigger_wiki_lint: () => [],
  get_wiki_lint_results: () => [],
  wiki_lint_keep: () => ({ success: true }),
  wiki_lint_delete: () => ({ success: true }),
  wiki_lint_recompile: () => ({ success: true }),

  // ----- Wiki 页面源 -----
  get_page_sources: () => [],
  get_content_wiki_pages: () => [],

  // ----- 编译 -----
  compile_content_to_wiki: () => [],

  // ----- 自动化 -----
  get_automation_status: () => ({ enabled: false, last_run: null }),
  request_automation_permission: () => ({ success: true }),
  dismiss_automation_prompt: () => ({ success: true }),
  open_automation_settings: () => ({ success: true }),

  // ----- 报告 -----
  generate_report: () => ({ success: true, path: '/tmp/report.md' }),
  get_report: () => null,
  get_all_reports: () => [],
  submit_feedback: () => ({ success: true }),

  // ----- 更新 -----
  check_for_update_manual: () => null,
  set_update_check_enabled: () => ({ success: true }),
  get_update_settings: () => ({ check_enabled: true, last_check: null }),

  // ----- OAuth -----
  logout_openai_oauth: () => ({ success: true }),
  logout_gemini_oauth: () => ({ success: true }),

  // ----- 剪贴板 / 摘要 -----
  copy_content_summary: () => ({ success: true }),
};

// ===================================================================
// 通用命令 — 按名称推断模式
// ===================================================================
function inferHandler(cmd: string): CommandHandler | null {
  // get_ → 返回空数组
  if (cmd.startsWith('get_')) {
    return () => [];
  }
  // create_ / update_ / delete_ / save_ / set_ → { success: true }
  if (
    cmd.startsWith('create_') ||
    cmd.startsWith('update_') ||
    cmd.startsWith('delete_') ||
    cmd.startsWith('save_') ||
    cmd.startsWith('set_') ||
    cmd.startsWith('dismiss_') ||
    cmd.startsWith('ignore_') ||
    cmd.startsWith('unlink_') ||
    cmd.startsWith('link_') ||
    cmd.startsWith('open_') ||
    cmd.startsWith('copy_') ||
    cmd.startsWith('logout_')
  ) {
    return () => ({ success: true });
  }
  // search_ → 空数组
  if (cmd.startsWith('search_')) {
    return () => [];
  }
  // check_ / test_ → null
  if (cmd.startsWith('check_') || cmd.startsWith('test_')) {
    return () => null;
  }
  // generate_ / trigger_ / record_ / submit_ → 通用成功
  if (
    cmd.startsWith('generate_') ||
    cmd.startsWith('trigger_') ||
    cmd.startsWith('record_') ||
    cmd.startsWith('submit_') ||
    cmd.startsWith('accept_') ||
    cmd.startsWith('export_') ||
    cmd.startsWith('import_') ||
    cmd.startsWith('compile_') ||
    cmd.startsWith('retry_') ||
    cmd.startsWith('ocr_') ||
    cmd.startsWith('auto_')
  ) {
    return () => ({ success: true });
  }

  return null;
}

// ===================================================================
// 所有命令处理器映射
// ===================================================================
export const commandHandlers: Record<string, CommandHandler> = {
  ...coreHandlers,
};

// ===================================================================
// Fallback 处理器
// ===================================================================
export function fallbackHandler(cmd: string): unknown {
  const inferred = inferHandler(cmd);
  if (inferred) {
    console.debug(`[mock:cmd-router] inferred handler for "${cmd}"`);
    return inferred({});
  }
  console.warn(`[mock:cmd-router] UNKNOWN command: "${cmd}" — returning null`);
  return null;
}

// ===================================================================
// 主路由函数
// ===================================================================
export function routeCommand<T>(
  cmd: string,
  args?: Record<string, unknown>
): T {
  const handler = commandHandlers[cmd];
  if (handler) {
    console.debug(`[mock:cmd-router] → handling "${cmd}"`);
    return handler(args) as T;
  }
  return fallbackHandler(cmd) as T;
}
