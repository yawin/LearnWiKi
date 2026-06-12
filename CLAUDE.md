Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.
**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**
Before implementing:

- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.
  Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**
When editing existing code:

- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.
  When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.
  The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**
Transform tasks into verifiable goals:

- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"
  For multi-step tasks, state a brief plan:

```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

## Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

## **These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.

# Project: LearnWiki

## Permissions

- You may run any shell commands needed for development (npm, cargo, npx, etc.)
- You may read, write, and edit any files in this project
- You may run `npm run dev`, `npm run build`, `cargo build`, `cargo check`, `cargo tauri dev` and similar development commands freely
- You may install npm packages and cargo crates as needed
- You may run tests freely
- You may create, delete, and modify files within this project directory

## Decision Protocol

- For routine development tasks (coding, debugging, refactoring, testing): proceed autonomously, no need to ask
- For important project direction decisions (architecture changes, new major dependencies, data model redesign, feature scope changes): ask the user first, present 2-3 options with a recommended choice clearly marked
- If the user does not respond within 3 minutes: proceed with the recommended option and note what was chosen
- The user is a beginner — always explain decisions in plain Chinese, avoid jargon

## GitHub

- Repository: https://github.com/kdsz001/LearnWiki
- When user says "存一下" or "保存进度": commit + push to GitHub
- Write clear commit messages in conventional format (feat/fix/refactor)

## Release workflow

User is non-technical and doesn't maintain the `release-notes/` folder
or run any release commands manually. When user says "发版", "发 release",
"打 tag", "发个新版本", or similar, Claude owns the full release flow.
**Read `release-notes/README.md` first** — it has the step-by-step
checklist (pick version number, summarize commits into user-facing
bullets, create `release-notes/vX.Y.Z.md` from TEMPLATE.md, bump the
three version files, run `cargo check` to sync Cargo.lock, commit, tag,
push, push tag). After pushing the tag, report back with the Actions
URL so the user can watch the DMG build.

The key rule: never paste raw commit messages into release notes.
Rewrite everything into short user-facing sentences like "优化了 X" or
"修复了 X", no technical details.

## Project Info

- Tauri 2 desktop app (Rust + React/TypeScript)
- Frontend: React 19, Tailwind CSS 4, Zustand, Framer Motion
- Backend: Rust with SQLite (via rusqlite)
- Package manager: npm
- Build: Vite 6

## Design System

Always read DESIGN.md before making any visual or UI decisions.
All font choices, colors, spacing, and aesthetic direction are defined there.
Do not deviate without explicit user approval.
In QA mode, flag any code that doesn't match DESIGN.md.

## Architecture & Conventions

### Key Docs

- Product design: `docs/learn-system-requirement.md` (727 lines)
- Coding standards: `docs/coding-standards.md` (includes test rules, all sections)

### Coding Conventions

- File names: `kebab-case` | Components: `PascalCase` | Functions/Vars: `camelCase`
- CSS: Tailwind utility classes only, no custom CSS classes
- State: Zustand with precise selectors (avoid full-store destructuring)
- Props: explicit interface, no inline types
- Exports: named exports only, no `default export`
- Tests: Vitest (frontend) + `cargo test` (backend)
- Commits: conventional format (`feat:`/`fix:`/`refactor:`/`docs:`/`chore:`)

### Tauri Plugin Checklist (MUST verify 4 places)

When adding, removing, or modifying any Tauri plugin, ALL 4 locations must be updated.
Missing any one causes silent runtime failure — the plugin simply does nothing with no error.

| # | Location | What to add | Failure mode if missing |
|---|---|---|---|
| 1 | `package.json` | `@tauri-apps/plugin-xxx` (npm dep) | `import()` throws "module not found" |
| 2 | `src-tauri/Cargo.toml` | `tauri-plugin-xxx = "2"` (Rust crate) | `cargo check` fails on unresolved import |
| 3 | `src-tauri/src/lib.rs` | `.plugin(tauri_plugin_xxx::init())` in Builder chain | **Silent failure** — calls go nowhere, no error |
| 4 | `src-tauri/capabilities/default.json` | Corresponding permission (e.g. `"dialog:default"`) | Tauri runtime throws permission-denied error |

Item 3 is the most dangerous — it passes all compile-time checks and fails silently at runtime.
After adding any plugin, run `npm run tauri dev` and manually verify its functionality works.

### Pre-Commit Checklist

Before ANY `git commit`, run these 4 checks. Do NOT skip any:

```bash
npx tsc -b              # MUST use -b (not --noEmit) — catches strict project reference errors
npx vitest run          # all tests must pass
cargo check --manifest-path src-tauri/Cargo.toml  # Rust compilation
```

After committing, CodeGraph auto-indexes via a PostToolUse hook. If querying CodeGraph within 2 seconds of a commit, run `codegraph index` manually — the file watcher has ~500ms debounce delay.

### Async Data Null-Safety Rule

Any component that loads async data into `useState<T | null>` MUST handle the null path.
When modifying such a component, you MUST add a test case: mock the data source to fail/resolve-empty,
assert the component renders fallback UI (not a blank white page from a React crash).

```tsx
// ❌ Crashes when trail is null
{trail.schedule ? <MasteryBar /> : null}

// ✅ Safe
{trail?.schedule ? <MasteryBar /> : <Fallback text="数据不可用" />}
```

Common crash patterns (accessing `.field` on null):
- API response → `setData(null)` on error → render accesses `data.field` → TypeError → blank
- Array from API → `data.items.map(...)` → `data` is null → blank

The existing test `GoalDetail.test.tsx` (mock at Tauri invoke level, verify title renders) is the reference pattern.

### Cross-Component Event Rule

Every `window.dispatchEvent(new CustomEvent("event-name", ...))` MUST have a corresponding
`window.addEventListener("event-name", ...)` in the same commit, plus a test that fires the
event and asserts the listener was called.

```tsx
// In the same commit:
// 1. Dispatch site:
window.dispatchEvent(new CustomEvent("open-wiki-detail", { detail: { wikiPageId } }));

// 2. Listen site:
useEffect(() => {
  const handler = (e) => selectPage(e.detail.wikiPageId);
  window.addEventListener("open-wiki-detail", handler);
  return () => window.removeEventListener("open-wiki-detail", handler);
}, []);

// 3. Test:
it("dispatches event on click", () => {
  const fn = vi.fn();
  window.addEventListener("open-wiki-detail", fn);
  fireEvent.click(screen.getByText("..."));
  expect(fn).toHaveBeenCalled();
});
```

### Current Development

- Epic: E-1 (Foundation) — Phase 1
- Next: Start E-1-1 (Data model migration)
- Communicate with user in Chinese
- Parallel dev: 2-3 Claude agents per batch via worktrees

## Skill routing

- Product ideas, "is this worth building", brainstorming → invoke office-hours
- Bugs, errors, "why is this broken", 500 errors → invoke investigate
- Ship, deploy, push, create PR → invoke ship
- QA, test the site, find bugs → invoke qa
- Code review, check my diff → invoke review
- Update docs after shipping → invoke document-release
- Weekly retro → invoke retro
- Design system, brand → invoke design-consultation
- Visual audit, design polish → invoke design-review
- Architecture review → invoke plan-eng-review
