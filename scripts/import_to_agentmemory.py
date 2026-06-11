#!/usr/bin/env python3
"""Create AgentMemory observations from OpenWiki data for graph visualization."""
import json, sqlite3, subprocess, sys, time

DB = "/Users/macbook/Library/Application Support/com.openwiki.app/openwiki.db"
API = "http://127.0.0.1:3111/agentmemory/observe"
BASE = {
    "hookType": "manual",
    "sessionId": "openwiki-import",
    "project": "OpenWiki",
    "cwd": "/Users/macbook/llmwiki/openwiki",
    "timestamp": "2026-06-08T00:00:00Z"
}

def observe(content, meta=None):
    payload = dict(BASE)
    payload["content"] = content
    if meta:
        payload["metadata"] = meta
    data = json.dumps(payload)
    cmd = ["curl", "-s", "-X", "POST", API,
           "-H", "Content-Type: application/json",
           "-d", data, "--max-time", "30"]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=35)
    ok = result.returncode == 0 and '"observationId"' in result.stdout
    return ok

try:
    conn = sqlite3.connect(DB)
    conn.row_factory = sqlite3.Row
    c = conn.cursor()
except Exception as e:
    print(f"❌ DB: {e}")
    sys.exit(1)

count = 0

# 1. Project overview
print("📊 Project overview...", end=" ")
if observe("OpenWiki is a Tauri 2 desktop app for AI-powered learning with wiki pages, learning paths, spaced repetition, and task management."):
    count += 1; print("✅")
else: print("❌")
time.sleep(2)

# 2. Wiki pages (batch)
print("📄 Wiki pages...", end=" ")
rows = c.execute("SELECT title, tags, summary FROM wiki_pages ORDER BY updated_at DESC LIMIT 15").fetchall()
wiki_text = " | ".join(f"{r['title']}(tags:{r['tags'] or 'none'})" for r in rows)
if observe(f"OpenWiki wiki pages: {wiki_text}", {"type": "wiki", "count": len(rows)}):
    count += 1; print("✅")
else: print("❌")
time.sleep(2)

# 3. Learning paths
print("📚 Learning paths...", end=" ")
rows = c.execute("SELECT title, topic, description FROM learning_paths").fetchall()
if rows:
    lp_text = " | ".join(f"{r['title']}({r['topic']})" for r in rows)
    if observe(f"OpenWiki learning paths: {lp_text}", {"type": "learning"}):
        count += 1; print("✅")
    else: print("❌")
else: print("(empty)")
time.sleep(2)

# 4. Tasks
print("🎯 Tasks...", end=" ")
tasks = c.execute("SELECT COUNT(*) FROM practice_tasks").fetchone()[0]
done = c.execute("SELECT COUNT(*) FROM practice_tasks WHERE status='completed'").fetchone()[0] if tasks else 0
if tasks:
    rows = c.execute("SELECT title, status FROM practice_tasks ORDER BY updated_at DESC LIMIT 10").fetchall()
    task_text = " | ".join(f"{r['title']}({r['status']})" for r in rows)
    if observe(f"OpenWiki tasks: {tasks} total, {done} completed. Recent: {task_text}", {"type": "task"}):
        count += 1; print("✅")
    else: print("❌")
else: print("(empty)")
time.sleep(2)

# 5. Reviews
print("🔄 Reviews...", end=" ")
reviews = c.execute("SELECT COUNT(*) FROM review_schedule WHERE is_archived=0").fetchone()[0]
if reviews:
    due = c.execute("SELECT COUNT(*) FROM review_schedule WHERE is_archived=0 AND next_review_at <= datetime('now')").fetchone()[0]
    if observe(f"OpenWiki review system: {reviews} active schedules, {due} due now. SM-2 spaced repetition with 7 review formats.", {"type": "review"}):
        count += 1; print("✅")
    else: print("❌")
else: print("(empty)")
time.sleep(2)

# 6. Task-Wiki links (relationships)
print("🔗 Links...", end=" ")
links = c.execute("SELECT COUNT(*) FROM task_wiki_links").fetchone()[0]
if links:
    rows = c.execute("""
        SELECT twl.link_type, pt.title as task_title, wp.title as wiki_title
        FROM task_wiki_links twl
        LEFT JOIN practice_tasks pt ON pt.id=twl.task_id
        LEFT JOIN wiki_pages wp ON wp.id=twl.wiki_page_id
        LIMIT 10
    """).fetchall()
    link_text = " | ".join(f"{r['task_title'] or '?'}--[{r['link_type']}]-->{r['wiki_title'] or '?'}" for r in rows)
    if observe(f"OpenWiki bidirectional links between tasks and wiki pages: {link_text}", {"type": "link"}):
        count += 1; print("✅")
    else: print("❌")
else: print("(empty)")
time.sleep(2)

# 7. Captured content
print("📥 Captured content...", end=" ")
cc = c.execute("SELECT COUNT(*) FROM captured_content").fetchone()[0]
if cc:
    rows = c.execute("SELECT title FROM captured_content ORDER BY created_at DESC LIMIT 5").fetchall()
    titles = [r['title'] or '(no title)' for r in rows]
    if observe(f"OpenWiki captured {cc} pieces of content. Latest: {' | '.join(titles)}", {"type": "content"}):
        count += 1; print("✅")
    else: print("❌")
else: print("(empty)")

conn.close()
print(f"\n✅ 创建了 {count} 条 AgentMemory 观察（observation）")
print("   图谱提取将在 consolidate 管道中自动运行")
print("   查看: http://127.0.0.1:3113/#graph")
