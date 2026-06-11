#!/bin/bash
# Import OpenWiki data into AgentMemory
# Uses a single aggregated memory per entity type to reduce API calls

DB=~/Library/Application\ Support/com.openwiki.app/openwiki.db
API=http://127.0.0.1:3111/agentmemory/remember

remember() {
  local content="$1"
  local metadata="$2"
  curl -s -X POST "$API" -H "Content-Type: application/json" \
    -d "{\"content\":$(echo "$content" | python3 -c "import json,sys; print(json.dumps(sys.stdin.read()))"), \"metadata\":$metadata}" \
    --max-time 10 >/dev/null 2>&1 && echo -n "." || echo -n "x"
}

echo "📄 Importing Wiki Pages..."
wiki_summary=$(sqlite3 "$DB" "SELECT COUNT(*) || ' pages' FROM wiki_pages")
remember "OpenWiki has $wiki_summary. 
Top pages: $(sqlite3 "$DB" "SELECT group_concat(title, ', ') FROM (SELECT title FROM wiki_pages ORDER BY updated_at DESC LIMIT 10)")" \
  '{"source":"openwiki","type":"wiki","summary":"all_pages"}'
echo ""

echo "📚 Importing Learning Paths..."
paths=$(sqlite3 "$DB" "SELECT COUNT(*) FROM learning_paths")
if [ "$paths" -gt 0 ]; then
  path_summary=$(sqlite3 "$DB" "SELECT group_concat(title || ' (' || topic || ')', ' | ') FROM learning_paths")
  remember "OpenWiki Learning Paths ($paths total): $path_summary" \
    '{"source":"openwiki","type":"learning","summary":"all_paths"}'
fi
echo ""

echo "🎯 Importing Tasks..."
tasks=$(sqlite3 "$DB" "SELECT COUNT(*) FROM practice_tasks")
if [ "$tasks" -gt 0 ]; then
  done_tasks=$(sqlite3 "$DB" "SELECT COUNT(*) FROM practice_tasks WHERE status='completed'")
  in_progress=$(sqlite3 "$DB" "SELECT COUNT(*) FROM practice_tasks WHERE status='in_progress'")
  remember "OpenWiki Practice Tasks: $tasks total ($done_tasks completed, $in_progress in progress)" \
    '{"source":"openwiki","type":"task","summary":"all_tasks"}'
fi
echo ""

echo "🔄 Importing Reviews..."
reviews=$(sqlite3 "$DB" "SELECT COUNT(*) FROM review_schedule WHERE is_archived=0")
if [ "$reviews" -gt 0 ]; then
  due=$(sqlite3 "$DB" "SELECT COUNT(*) FROM review_schedule WHERE is_archived=0 AND next_review_at <= datetime('now')")
  remember "OpenWiki Review Schedule: $reviews active reviews ($due due now)" \
    '{"source":"openwiki","type":"review","summary":"review_stats"}'
fi
echo ""

echo "⭐ Importing Recommendations..."
recs=$(sqlite3 "$DB" "SELECT COUNT(*) FROM task_recommendations")
if [ "$recs" -gt 0 ]; then
  top_rec=$(sqlite3 "$DB" "SELECT pt.title || ' (score:' || printf('%.2f',tr.score) || ')' FROM task_recommendations tr LEFT JOIN practice_tasks pt ON pt.id=tr.task_id ORDER BY tr.score DESC LIMIT 1")
  remember "OpenWiki Task Recommendations: $recs total. Top recommendation: $top_rec" \
    '{"source":"openwiki","type":"recommendation","summary":"all_recommendations"}'
fi
echo ""

echo "🔗 Importing Links..."
links=$(sqlite3 "$DB" "SELECT COUNT(*) FROM task_wiki_links")
if [ "$links" -gt 0 ]; then
  remember "OpenWiki Task-Wiki Links: $links bidirectional links between tasks and wiki pages" \
    '{"source":"openwiki","type":"link","summary":"all_links"}'
fi
echo ""

echo "💡 Importing Solutions..."
solutions=$(sqlite3 "$DB" "SELECT COUNT(*) FROM task_solutions")
if [ "$solutions" -gt 0 ]; then
  remember "OpenWiki Task Solutions: $solutions reference solutions with author attribution" \
    '{"source":"openwiki","type":"solution","summary":"all_solutions"}'
fi
echo ""

echo "📊 Importing Stats..."
stats=$(sqlite3 "$DB" "SELECT group_concat(name || ':' || cnt, ', ') FROM (SELECT 'wiki_pages' as name, COUNT(*) as cnt FROM wiki_pages UNION ALL SELECT 'captured_content', COUNT(*) FROM captured_content UNION ALL SELECT 'review_logs', COUNT(*) FROM review_logs UNION ALL SELECT 'chat_messages', COUNT(*) FROM chat_messages)")
remember "OpenWiki Database Stats: $stats" \
  '{"source":"openwiki","type":"stats","summary":"database_stats"}'
echo ""

echo ""
echo "✅ Done! Open http://127.0.0.1:3113/#graph to see the knowledge graph"
