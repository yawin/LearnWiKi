use crate::commands::capture::AppState;
use crate::storage::models::{
    AdaptiveRecommendation,
    ClozeBlank, ClozeQuestion, ComprehensiveQuiz, ComprehensiveQuizQuestion,
    DailyReviewSummary, DueReviewItem, ErrorHuntQuestion,
    Exam, ExamDetail, ExamQuestion, ExamSummary,
    ExplainFeedback, ExplainQuestion, HealthTrailResult, OrderingSteps, QuestionResult, QuizAnswer,
    QuizQuestion, QuizResult, ReviewHealthItem, ReviewHealthStats, ReviewSchedule,
    VariantQuestion, WeakPageInfo, WikiPage,
};
use rusqlite::params;
use crate::storage::repository::Repository;
use tauri::State;

#[tauri::command]
pub fn get_health_schedules(state: State<'_, AppState>) -> Result<Vec<ReviewHealthItem>, String> {
    let repo = Repository::new(state.db.clone());
    let items = repo.get_all_review_schedules().map_err(|e| e.to_string())?;
    Ok(items
        .into_iter()
        .map(|(schedule, page)| ReviewHealthItem {
            schedule,
            wiki_title: page.title,
            wiki_summary: page.summary,
            tags: page.tags,
        })
        .collect())
}

#[tauri::command]
pub fn get_review_health_stats(state: State<'_, AppState>) -> Result<ReviewHealthStats, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_review_health_stats().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_wiki_learning_trail(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<HealthTrailResult, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_wiki_learning_trail(&wiki_page_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_review_schedule(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<ReviewSchedule, String> {
    let repo = Repository::new(state.db.clone());
    repo.create_review_schedule(&wiki_page_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_due_reviews(
    state: State<'_, AppState>,
    limit: Option<i32>,
) -> Result<Vec<DueReviewItem>, String> {
    let repo = Repository::new(state.db.clone());
    let l = limit.unwrap_or(20) as i64;
    repo.get_due_reviews(l).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_due_review_for_page(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<Option<DueReviewItem>, String> {
    log::info!("[singleReview] get_due_review_for_page called with wiki_page_id: {}", wiki_page_id);
    let repo = Repository::new(state.db.clone());
    let result = repo.get_due_review_for_page(&wiki_page_id).map_err(|e| e.to_string());
    log::info!("[singleReview] get_due_review_for_page result: {:?}", result.as_ref().map(|o| o.is_some()));
    result
}

#[tauri::command]
pub fn auto_create_review_schedule(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<ReviewSchedule, String> {
    log::info!("[singleReview] auto_create_review_schedule called with wiki_page_id: {}", wiki_page_id);
    let repo = Repository::new(state.db.clone());
    let existing = repo.get_review_schedule(&wiki_page_id).map_err(|e| e.to_string())?;
    match existing {
        Some(schedule) => {
            log::info!("[singleReview] schedule exists id: {}", schedule.id);
            Ok(schedule)
        },
        None => {
            log::info!("[singleReview] creating new schedule");
            repo.create_review_schedule(&wiki_page_id).map_err(|e| e.to_string())
        },
    }
}

#[tauri::command]
pub fn submit_review_feedback(
    state: State<'_, AppState>,
    schedule_id: String,
    quality: i32,
    review_format: Option<String>,
    response_time_seconds: Option<i32>,
    session_id: Option<String>,
    question_snapshot: Option<String>,
) -> Result<ReviewSchedule, String> {
    let repo = Repository::new(state.db.clone());
    repo.submit_review_feedback_with_format(&schedule_id, quality, review_format.as_deref(), response_time_seconds, session_id.as_deref(), question_snapshot.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_review_stats(state: State<'_, AppState>) -> Result<(i32, i32, i32), String> {
    let repo = Repository::new(state.db.clone());
    repo.get_review_stats().map_err(|e| e.to_string())
}

// ========== Sprint 5A: Quiz & Ordering Generation ==========

/// Generate quiz questions from wiki page content using LLM.
#[tauri::command]
pub async fn generate_quiz_questions(
    state: State<'_, AppState>,
    wiki_page_id: String,
    count: Option<i32>,
) -> Result<Vec<QuizQuestion>, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo
        .get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let content = format!(
        "标题：{}\n摘要：{}\n正文：{}\n标签：{}",
        page.title,
        page.summary.as_deref().unwrap_or(""),
        page.body_markdown,
        page.tags.as_deref().unwrap_or(""),
    );
    let n = count.unwrap_or(1);
    let locale = crate::locale::resolve_locale(&state.db);

    let system_prompt = if crate::locale::is_english(&locale) {
        format!(
            "You are a quiz generator. Generate {} multiple-choice question(s) with 4 options each, \
             based on the knowledge content provided. Each question must have exactly one correct answer.\n\
             Return a JSON array: [{{\"stem\":\"question text\",\"options\":[\"A\",\"B\",\"C\",\"D\"],\"correct_index\":0,\"explanation\":\"why this answer is correct\"}}]",
            n
        )
    } else {
        format!(
            "你是一个出题助手。根据以下知识点内容，生成 {} 道选择题（每题 4 个选项），\
             每道题有且仅有一个正确答案。\
             返回 JSON 数组：[{{\"stem\":\"题干\",\"options\":[\"A\",\"B\",\"C\",\"D\"],\"correct_index\":0,\"explanation\":\"为什么这个答案正确\"}}]",
            n
        )
    };

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &content,
        2048,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    // Parse as array
    let questions: Vec<QuizQuestion> = if let Some(arr) = json.as_array() {
        arr.iter().map(|item| {
            QuizQuestion {
                stem: item.get("stem").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                options: item.get("options").and_then(|v| v.as_array()).map(|arr| {
                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
                }).unwrap_or_default(),
                correct_index: item.get("correct_index").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                explanation: item.get("explanation").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            }
        }).collect()
    } else if let Some(obj) = json.as_object() {
        // Single question wrapped in object
        vec![QuizQuestion {
            stem: obj.get("stem").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            options: obj.get("options").and_then(|v| v.as_array()).map(|arr| {
                arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
            }).unwrap_or_default(),
            correct_index: obj.get("correct_index").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            explanation: obj.get("explanation").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }]
    } else {
        vec![]
    };

    if questions.is_empty() {
        return Err("Failed to generate valid quiz questions from LLM response".to_string());
    }

    Ok(questions)
}

/// Generate ordering steps from wiki page content using LLM.
#[tauri::command]
pub async fn generate_ordering_steps(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<OrderingSteps, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo
        .get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let content = format!(
        "标题：{}\n摘要：{}\n正文：{}\n标签：{}",
        page.title,
        page.summary.as_deref().unwrap_or(""),
        page.body_markdown,
        page.tags.as_deref().unwrap_or(""),
    );
    let locale = crate::locale::resolve_locale(&state.db);

    let system_prompt = if crate::locale::is_english(&locale) {
        r#"You are a quiz generator. Extract the core steps or流程 from the knowledge content below,
and arrange them in correct order. Each step should be 2-8 words.
Return JSON: {"title":"ordering title (e.g. 'Put the steps of XX in correct order')","steps":["step 1","step 2","step 3",...]}
Requirements: 4-7 steps, each must be grounded in the content."#.to_string()
    } else {
        r#"你是一个出题助手。根据以下知识点内容，提取核心步骤或流程，
按照正确顺序排列，每步 2-8 个字。
返回 JSON：
{
  "title": "排序标题（如"把 XX 的步骤排序正确"）",
  "steps": ["第一步", "第二步", "第三步", ...]
}
知识点：{content}
要求：步骤数 4-7 项，每项必须能从知识点中找到依据。"#.to_string()
    };

    let user_msg = format!(
        "知识点内容：\n{}\n\n请输出 JSON 格式的排序题目。",
        content
    );

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &user_msg,
        2048,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let title = json
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| {
            if crate::locale::is_english(&locale) {
                "Put the steps in correct order"
            } else {
                "把步骤排序正确"
            }
        })
        .to_string();

    let steps: Vec<String> = json
        .get("steps")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    if steps.len() < 2 {
        return Err("Failed to generate valid ordering steps from LLM response".to_string());
    }

    Ok(OrderingSteps {
        title,
        correct_order: steps,
    })
}

/// Get the list of available review formats.
#[tauri::command]
pub fn get_available_formats() -> Result<Vec<String>, String> {
    Ok(vec![
        "quiz".to_string(),
        "matching".to_string(),
        "rapid_fire".to_string(),
        "ordering".to_string(),
        "error_hunt".to_string(),
        "cloze".to_string(),
        "explain".to_string(),
    ])
}

/// Generate an error hunt (find the mistake) question from wiki page content using LLM.
#[tauri::command]
pub async fn generate_error_hunt(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<ErrorHuntQuestion, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo
        .get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let content = format!(
        "标题：{}\n摘要：{}\n正文：{}\n标签：{}",
        page.title,
        page.summary.as_deref().unwrap_or(""),
        page.body_markdown,
        page.tags.as_deref().unwrap_or(""),
    );

    let system_prompt = "你是一个出题助手。根据以下知识点内容，生成一道\"找茬\"题。要求：

1. 写一段包含 1 个关键错误的描述（不能太明显，需要有合理性）
2. 提供 4 个选项，让用户选出错误在哪里
3. 提供解析和正确版本

返回 JSON 格式（不要 markdown 包裹）：
{
  \"content\": \"带错误的描述文本\",
  \"error_index\": 0,
  \"options\": [\"选项A\", \"选项B\", \"选项C\", \"选项D\"],
  \"explanation\": \"为什么错了、正确的应该是什么\",
  \"correct_version\": \"修正后的正确版本\"
}".to_string();

    let user_msg = format!(
        "知识点内容：\n{}\n\n请输出 JSON 格式的找茬题。",
        content
    );

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &user_msg,
        2048,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let question = ErrorHuntQuestion {
        title: page.title.clone(),
        content: json
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        error_index: json
            .get("error_index")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32,
        options: json
            .get("options")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
        explanation: json
            .get("explanation")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        correct_version: json
            .get("correct_version")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
    };

    if question.content.is_empty() || question.options.len() != 4 {
        return Err("Failed to generate valid error hunt question from LLM response".to_string());
    }

    Ok(question)
}

/// Generate a cloze (fill-in-the-blank) question from wiki page content using LLM.
#[tauri::command]
pub async fn generate_cloze(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<ClozeQuestion, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo
        .get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let content = format!(
        "标题：{}\n摘要：{}\n正文：{}\n标签：{}",
        page.title,
        page.summary.as_deref().unwrap_or(""),
        page.body_markdown,
        page.tags.as_deref().unwrap_or(""),
    );

    let system_prompt = "你是一个出题助手。根据以下知识点内容，生成一道填空题。要求：

1. 提取核心定义或流程，把 1-2 个关键术语替换为 ___
2. 每个空提供正确答案和 1-2 个同义词（模糊匹配用）
3. 可选提供提示

返回 JSON 格式（不要 markdown 包裹）：
{
  \"template\": \"RAG（Retrieval-Augmented Generation）是一种将___与___结合的方法。\",
  \"blanks\": [
    {\"index\": 0, \"correct_answers\": [\"检索\", \"信息检索\", \"搜索\"], \"hint\": \"从知识库找信息\"},
    {\"index\": 1, \"correct_answers\": [\"生成\", \"文本生成\", \"大语言模型生成\"], \"hint\": \"LLM 做的事\"}
  ]
}".to_string();

    let user_msg = format!(
        "知识点内容：\n{}\n\n请输出 JSON 格式的填空题。",
        content
    );

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &user_msg,
        2048,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let template = json
        .get("template")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Parse blanks directly
    let blanks: Vec<ClozeBlank> = json
        .get("blanks")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let index = item.get("index").and_then(|v| v.as_u64())? as usize;
                    let correct_answers: Vec<String> = item
                        .get("correct_answers")
                        .and_then(|v| v.as_array())?
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    let hint = item.get("hint").and_then(|v| v.as_str()).map(|s| s.to_string());
                    Some(ClozeBlank {
                        index,
                        correct_answers,
                        hint,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    if template.is_empty() || blanks.is_empty() {
        return Err("Failed to generate valid cloze question from LLM response".to_string());
    }

    Ok(ClozeQuestion { template, blanks })
}

pub(crate) fn generate_slug(text: &str) -> String {
    let mut slug = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            slug.push(ch);
        } else if ch == ' ' || ch == '：' || ch == ':' {
            slug.push('-');
        } else if ch >= '\u{4e00}' && ch <= '\u{9fff}' {
            for byte in ch.to_string().as_bytes() {
                slug.push_str(&format!("{:02x}", byte));
            }
        }
    }
    let slug = slug.trim_matches('-').to_string();
    if slug.len() > 100 { slug[..100].to_string() } else { slug }
}

// ========== Sprint 6A: Explain (E-6-1) ==========

/// Generate an "explain like I'm 5" review question from wiki page content using LLM.
#[tauri::command]
pub async fn generate_explain_review(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<ExplainQuestion, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo
        .get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let content = format!(
        "标题：{}\n摘要：{}\n正文：{}\n标签：{}",
        page.title,
        page.summary.as_deref().unwrap_or(""),
        page.body_markdown,
        page.tags.as_deref().unwrap_or(""),
    );

    let system_prompt = "你是一个友好的导师。根据以下知识点，生成一个引导用户用自己的话解释概念的场景。\n\
     \n\
    要求：\n\
    1. 用自然、友好的语气让用户解释这个概念（就像让朋友解释给他听）\n\
    2. 可以给一个生活化的提示帮助用户开始\n\
    \n\
    返回 JSON 格式（不要 markdown 包裹）：\n\
    {\n\
      \"prompt\": \"请用你自己的话解释一下什么是...？\",\n\
      \"hint\": \"想象一下...\"\n\
    }".to_string();

    let user_msg = format!(
        "知识点内容：\n{}\n\n请输出 JSON 格式的解释题。",
        content
    );

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &user_msg,
        2048,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let prompt = json
        .get("prompt")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let hint = json
        .get("hint")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if prompt.is_empty() {
        return Err("Failed to generate valid explain review from LLM response".to_string());
    }

    // Parse tags
    let tags: Vec<String> = page
        .tags
        .as_deref()
        .map(|t| {
            serde_json::from_str::<Vec<String>>(t).unwrap_or_default()
        })
        .unwrap_or_default();

    Ok(ExplainQuestion {
        wiki_title: page.title,
        wiki_summary: page.summary.unwrap_or_default(),
        wiki_tags: tags,
        prompt,
        hint,
    })
}

/// Submit user's explanation and get LLM feedback.
#[tauri::command]
pub async fn submit_explain_answer(
    state: State<'_, AppState>,
    wiki_page_id: String,
    user_explanation: String,
) -> Result<ExplainFeedback, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo
        .get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let content = format!(
        "标题：{}\n摘要：{}\n正文：{}\n标签：{}",
        page.title,
        page.summary.as_deref().unwrap_or(""),
        page.body_markdown,
        page.tags.as_deref().unwrap_or(""),
    );

    let system_prompt = "你是一位耐心的老师。用户正在用自己的话解释一个概念。\n\
    请评估他们的理解程度并给出建设性反馈。\n\
    \n\
    评估规则：\n\
    - 5分（大师）：解释准确、完整，有自己的理解，能举出合适例子\n\
    - 4分（良好）：解释基本正确，有一些自己的见解，但可以更完整\n\
    - 3分（合格）：核心概念说对了，但比较浅或不够清晰\n\
    - 2分（入门）：说到了一些相关的内容，但主要概念没说清楚\n\
    - 1分（新手）：解释不准确或与概念无关\n\
    \n\
    返回 JSON 格式（不要 markdown 包裹）：\n\
    {\n\
      \"score\": 4,\n\
      \"score_label\": \"良好\",\n\
      \"strength_points\": [\"你准确理解了核心概念\", \"例子举得很好\"],\n\
      \"weakness_points\": [\"可以更具体地说明细节\"],\n\
      \"improvement_suggestions\": [\"可以分步骤解释...\", \"试试用一个具体场景来说明\"],\n\
      \"better_example\": \"可以把概念想象成...\"\n\
    }".to_string();

    let user_msg = format!(
        "知识点内容：\n{}\n\n用户的解释：\n{}\n\n请输出 JSON 格式的评估反馈。",
        content, user_explanation
    );

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &user_msg,
        2048,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let score = json.get("score").and_then(|v| v.as_i64()).unwrap_or(3) as i32;
    let score_label = json.get("score_label").and_then(|v| v.as_str()).unwrap_or("合格").to_string();
    let strength_points: Vec<String> = json.get("strength_points")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let weakness_points: Vec<String> = json.get("weakness_points")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let improvement_suggestions: Vec<String> = json.get("improvement_suggestions")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    let better_example = json.get("better_example")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(ExplainFeedback {
        score,
        score_label,
        improvement_suggestions,
        better_example,
        strength_points,
        weakness_points,
    })
}

// ========== Sprint 6A: Variant Question (E-6-3) ==========

/// Generate a variant question from a different angle when user answered wrong.
#[tauri::command]
pub async fn generate_variant_question(
    state: State<'_, AppState>,
    wiki_page_id: String,
    current_format: String,
    variant_generation: i32,
) -> Result<VariantQuestion, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo
        .get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let content = format!(
        "标题：{}\n摘要：{}\n正文：{}\n标签：{}",
        page.title,
        page.summary.as_deref().unwrap_or(""),
        page.body_markdown,
        page.tags.as_deref().unwrap_or(""),
    );

    let system_prompt = format!(
        "你是一个出题助手。用户在上一次关于以下知识点的题目中答错了。\n\
        请从不同的角度生成一道变体题，变化表述方式但考查同一核心知识点。\n\
        这是第 {} 代变体（数字越大，变化应越大）。\n\
        \n\
        原题型：{}\n\
        \n\
        要求：\n\
        1. 考查同一核心知识点\n\
        2. 从不同角度提问（避免和原题一样）\n\
        3. 难度可以稍高或稍低\n\
        4. 返回和原题型格式一致的题目数据\n\
        \n\
        返回 JSON 格式（不要 markdown 包裹）：\n\
        {{\n\
          \"format\": \"{}\",\n\
          \"question_data\": {{ ... }},  // 根据题型不同而不同\n\
          \"twist_description\": \"这次我们从应用场景来考查\"\n\
        }}",
        variant_generation,
        current_format,
        current_format,
    );

    let user_msg = format!(
        "知识点内容：\n{}\n\n请输出 JSON 格式的变体题（原题型：{}）。",
        content, current_format
    );

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &user_msg,
        2048,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let fmt = json
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or(&current_format)
        .to_string();

    let question_data = json
        .get("question_data")
        .cloned()
        .unwrap_or(serde_json::Value::Null);

    let twist_description = json
        .get("twist_description")
        .and_then(|v| v.as_str())
        .unwrap_or("从不同角度考查同一知识点")
        .to_string();

    if question_data.is_null() {
        return Err("Failed to generate valid variant question from LLM response".to_string());
    }

    Ok(VariantQuestion {
        format: fmt,
        question_data,
        variant_generation,
        twist_description,
    })
}

// ========== Sprint 6B: Comprehensive Quiz (E-6-2) ==========

/// Generate a comprehensive quiz from multiple wiki pages / a learning path.
#[tauri::command]
pub async fn comprehensive_generate_quiz(
    state: State<'_, AppState>,
    wiki_page_ids: Vec<String>,
    learning_path_id: Option<String>,
    count: Option<i32>,
) -> Result<ComprehensiveQuiz, String> {
    let repo = Repository::new(state.db.clone());
    let n = count.unwrap_or(10).clamp(5, 30);
    let locale = crate::locale::resolve_locale(&state.db);

    let mut pages_content = String::new();
    let mut page_titles = Vec::new();
    let mut valid_ids = Vec::new();

    // Collect content: wiki_page_ids > learning_path > fallback all
    if !wiki_page_ids.is_empty() {
        for pid in &wiki_page_ids {
            if let Ok(Some(page)) = repo.get_wiki_page_by_id(pid) {
                pages_content.push_str(&format!(
                    "=== {} (标签: {}) ===\n{}\n\n",
                    page.title,
                    page.tags.as_deref().unwrap_or(""),
                    page.body_markdown,
                ));
                page_titles.push(page.title.clone());
                valid_ids.push(pid.clone());
            }
        }
    } else if let Some(lpid) = &learning_path_id {
        // Use learning path modules' theory content instead of all wiki pages
        let modules = repo.get_modules_by_path_id(lpid).map_err(|e| e.to_string())?;
        for m in &modules {
            if !m.theory_markdown.is_empty() {
                pages_content.push_str(&format!(
                    "=== {} ===\n{}\n\n",
                    m.title, m.theory_markdown,
                ));
                page_titles.push(m.title.clone());
                // Use module id as reference
                valid_ids.push(m.id.clone());
            }
        }
        if valid_ids.is_empty() {
            return Err("学习路径下没有可用的学习内容".to_string());
        }
    } else {
        // Fallback: get all wiki pages (limit to first 50 to avoid OOM)
        let pages = repo.get_all_wiki_pages(50, 0).map_err(|e| e.to_string())?;
        for page in &pages {
            let truncated: String = page.body_markdown.chars().take(2000).collect();
            pages_content.push_str(&format!(
                "=== {} (标签: {}) ===\n{}\n\n",
                page.title,
                page.tags.as_deref().unwrap_or(""),
                truncated,
            ));
            page_titles.push(page.title.clone());
            valid_ids.push(page.id.clone());
        }
    }

    if valid_ids.is_empty() {
        return Err("没有找到可用的知识库内容来生成测验".to_string());
    }

    // Limit content size to avoid token overflow (~8000 chars is safe for most models)
    if pages_content.len() > 8000 {
        pages_content = pages_content.chars().take(8000).collect::<String>()
            + "\n\n[内容已截断]";
    }

    let system_prompt = if crate::locale::is_english(&locale) {
        format!(
            "You are a comprehensive quiz generator. Generate {} mixed-type questions \
             (multiple choice, true/false, and short answer) based on the knowledge content below. \
             Questions should cover concepts, scenarios, and applications.\n\n\
             Type rules:\n\
             - \"choice\": 4 options, one correct (at least {} questions must be this type)\n\
             - \"true_false\": 2 options [\"True\", \"False\"], correct_index 0 or 1 (2-3 questions)\n\
             - \"short_answer\": 0 options (empty array), correct_index always 0 (at most 1)\n\n\
             Every question MUST include source_page_title matching a section title above.\n\n\
             Return JSON: {{\"title\":\"Quiz title\",\"questions\":[{{\"id\":1,\"stem\":\"...\",\"question_type\":\"choice\",\"options\":[\"A\",\"B\",\"C\",\"D\"],\"correct_index\":0,\"explanation\":\"...\",\"source_page_title\":\"...\"}}]}}",
            n,
            (n as f64 * 0.6).ceil() as i32,
        )
    } else {
        format!(
            "你是一个综合测验出题助手。根据以下知识库内容，生成 {} 道混编题（选择题+判断题+简答题），\
             覆盖概念理解、场景应用和实践应用三个层次。\n\n\
             题型分配规则：\n\
             - \"choice\": 4个选项的选择题（至少{}道）\n\
             - \"true_false\": 判断题，2个选项[\"对\", \"错\"]（2-3道）\n\
             - \"short_answer\": 简答题，0个选项（最多1道）\n\n\
             每道题必须注明来源页面（source_page_title字段）以便薄弱点追溯。\n\
             返回 JSON: {{\"title\":\"测验标题\",\"questions\":[{{\"id\":1,\"stem\":\"题干\",\"question_type\":\"choice\",\"options\":[\"A\",\"B\",\"C\",\"D\"],\"correct_index\":0,\"explanation\":\"解析\",\"source_page_title\":\"来源页面标题\"}}]}}",
            n,
            (n as f64 * 0.6).ceil() as i32,
        )
    };

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &pages_content,
        4096,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    // Validate AI response structure
    let title = json
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("综合测验")
        .to_string();

    let questions_raw = json
        .get("questions")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "AI 返回的 JSON 缺少 questions 数组".to_string())?;

    let mut questions: Vec<ComprehensiveQuizQuestion> = Vec::new();
    let mut choice_count = 0;
    let mut short_answer_count = 0;

    for (i, item) in questions_raw.iter().enumerate() {
        let qtype = item.get("question_type").and_then(|v| v.as_str()).unwrap_or("choice").to_string();
        let opts: Vec<String> = item.get("options").and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        // Validate correct_index bounds per type
        let raw_idx = item.get("correct_index").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let correct_index = match qtype.as_str() {
            "true_false" => raw_idx.clamp(0, 1),
            "choice" if opts.len() > 0 => raw_idx.clamp(0, opts.len() as i32 - 1),
            _ => raw_idx,
        };

        match qtype.as_str() {
            "choice" => choice_count += 1,
            "short_answer" => short_answer_count += 1,
            _ => {}
        }

        questions.push(ComprehensiveQuizQuestion {
            id: item.get("id").and_then(|v| v.as_i64()).unwrap_or(i as i64 + 1) as i32,
            stem: item.get("stem").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            question_type: qtype,
            options: opts,
            correct_index,
            explanation: item.get("explanation").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            source_page_title: item.get("source_page_title").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        });
    }

    // Validate type constraints
    if choice_count < 1 && short_answer_count as i32 == n {
        return Err("AI 返回的题型不符合要求，请重试".to_string());
    }

    // Filter out empty questions
    questions.retain(|q| !q.stem.is_empty());

    if questions.is_empty() {
        return Err("AI 未能生成有效的测验题目，请重试".to_string());
    }

    Ok(ComprehensiveQuiz {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        questions,
        source_page_ids: valid_ids,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Submit quiz answers and get results with weak point analysis.
#[tauri::command]
pub async fn comprehensive_submit_quiz(
    state: State<'_, AppState>,
    quiz: ComprehensiveQuiz,
    answers: Vec<QuizAnswer>,
) -> Result<QuizResult, String> {
    let repo = Repository::new(state.db.clone());
    let locale = crate::locale::resolve_locale(&state.db);
    let mut question_results = Vec::new();
    let mut correct_count = 0;
    let total = quiz.questions.len() as i32;

    for q in &quiz.questions {
        let answer = answers.iter().find(|a| a.question_id == q.id);
        let selected = answer.map(|a| a.selected_index).unwrap_or(-1);
        let short_text = answer.and_then(|a| a.short_answer_text.clone());

        // short_answer: not auto-gradable, always marked as incorrect (needs manual review)
        let is_correct = if q.question_type == "short_answer" {
            false
        } else {
            // Validate correct_index bounds for safe comparison
            let valid_max = match q.question_type.as_str() {
                "true_false" => 1i32.min(q.options.len() as i32 - 1).max(0),
                _ => (q.options.len() as i32 - 1).max(0),
            };
            let clamped_correct = q.correct_index.clamp(0, valid_max);
            selected == clamped_correct
        };

        if is_correct {
            correct_count += 1;
        }

        question_results.push(QuestionResult {
            question_id: q.id,
            stem: q.stem.clone(),
            correct: is_correct,
            correct_index: q.correct_index,
            selected_index: selected,
            explanation: q.explanation.clone(),
            source_page_id: None,
            source_page_title: Some(q.source_page_title.clone()),
            short_answer_text: short_text,
        });
    }

    let score_percent = if total > 0 {
        (correct_count as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    // Identify weak pages from wrong answers that have source_page_title
    let mut weak_pages: Vec<WeakPageInfo> = Vec::new();
    let mut page_wrong_count: std::collections::HashMap<String, (String, i32)> = std::collections::HashMap::new();
    for qr in &question_results {
        if !qr.correct {
            if let Some(ref src) = qr.source_page_title {
                if !src.is_empty() {
                    let entry = page_wrong_count.entry(src.clone()).or_insert_with(|| (src.clone(), 0));
                    entry.1 += 1;
                    // Auto-create review schedule for wrong answers — only if a real wiki_page_id exists
                    if let Some(ref page_id) = qr.source_page_id {
                        let _ = repo.create_review_schedule(page_id);
                    }
                }
            }
        }
    }
    for (_title, (page_title, count)) in &page_wrong_count {
        weak_pages.push(WeakPageInfo {
            page_id: String::new(), // we don't have the page id from source_page_title alone
            page_title: page_title.clone(),
            wrong_count: *count,
            total_related: 1,
        });
    }

    let mut suggestions = Vec::new();
    if score_percent < 60.0 {
        if crate::locale::is_english(&locale) {
            suggestions.push("Consider reviewing the relevant knowledge before trying again.".to_string());
        } else {
            suggestions.push("建议重新学习相关知识点后再尝试".to_string());
        }
    } else if score_percent < 80.0 {
        if crate::locale::is_english(&locale) {
            suggestions.push("Good foundation! Review the specific areas where you made mistakes.".to_string());
        } else {
            suggestions.push("基础不错，建议针对错题知识点进行专项复习".to_string());
        }
    } else {
        if crate::locale::is_english(&locale) {
            suggestions.push("Excellent! You're ready to move on to advanced topics.".to_string());
        } else {
            suggestions.push("掌握情况良好！可以继续进阶学习".to_string());
        }
    }
    if correct_count < total {
        let wrong_count = total - correct_count;
        if crate::locale::is_english(&locale) {
            suggestions.push(format!("{} wrong answers have been added to your review schedule.", wrong_count));
        } else {
            suggestions.push(format!("错题已自动加入复习计划（{} 道）", wrong_count));
        }
    }

    Ok(QuizResult {
        total,
        correct: correct_count,
        score_percent,
        answers: question_results,
        weak_pages,
        review_suggestions: suggestions,
    })
}

// ========== Sprint 6B: Adaptive Learning Recommendations (E-6-4) ==========

/// Get adaptive learning path recommendations based on review performance.
#[tauri::command]
pub async fn adaptive_get_recommendations(
    state: State<'_, AppState>,
) -> Result<Vec<AdaptiveRecommendation>, String> {
    let repo = Repository::new(state.db.clone());
    let locale = crate::locale::resolve_locale(&state.db);
    let mut recommendations = Vec::new();

    // Get all review schedules to find weak areas
    let review_data = repo.get_all_review_schedules().map_err(|e| e.to_string())?;

    // Get learning paths
    let paths = repo.get_all_learning_paths().map_err(|e| e.to_string())?;

    // Find weak areas: schedules with low ease factor or low variant streak
    let mut weak_pages_info = Vec::new();
    for (schedule, page) in &review_data {
        if schedule.variant_streak < 2 || schedule.ease_factor < 2.0 {
            weak_pages_info.push((page.title.clone(), page.id.clone()));
        }
    }

    // Recommendation 1: Strengthen weak areas
    if !weak_pages_info.is_empty() {
        let (title, id) = &weak_pages_info[0];
        let desc = if crate::locale::is_english(&locale) {
            format!("You've been struggling with \"{}\". Practice more to strengthen.", title)
        } else {
            format!("你在「{}」上答错率较高，建议加强练习", title)
        };
        recommendations.push(AdaptiveRecommendation {
            recommendation_type: "strengthen".to_string(),
            title: if crate::locale::is_english(&locale) { "Strengthen Weak Areas".to_string() } else { "强化薄弱环节".to_string() },
            description: desc,
            reason: if crate::locale::is_english(&locale) { format!("{} has low review accuracy", title) } else { format!("「{}」的复习正确率较低", title) },
            target_id: Some(id.clone()),
            target_type: "wiki_page".to_string(),
            priority: 1,
            learning_path_id: None,
            learning_path_name: None,
        });
    }

    // Recommendation 2: Advance to next path if high mastery
    for path in &paths {
        let modules = repo.get_modules_by_path_id(&path.id).map_err(|e| e.to_string())?;
        let mut total_tasks = 0i32;
        let mut completed_tasks = 0i32;
        for module in &modules {
            let tasks = repo.get_tasks_by_module_id(&module.id).map_err(|e| e.to_string())?;
            total_tasks += tasks.len() as i32;
            completed_tasks += tasks.iter().filter(|t| t.status == "completed").count() as i32;
        }
        if total_tasks == 0 {
            continue;
        }
        let mastery = completed_tasks as f64 / total_tasks as f64 * 100.0;

        if mastery >= 90.0 {
            let desc = if crate::locale::is_english(&locale) {
                format!("You've mastered \"{}\" ({}%). Ready for the next level!", path.title, mastery as i32)
            } else {
                format!("你已掌握「{}」（{}%），可以进入下一阶段！", path.title, mastery as i32)
            };
            recommendations.push(AdaptiveRecommendation {
                recommendation_type: "advance".to_string(),
                title: if crate::locale::is_english(&locale) { "Ready to Advance".to_string() } else { "可以进阶了".to_string() },
                description: desc,
                reason: if crate::locale::is_english(&locale) { format!("{}% mastery achieved", mastery as i32) } else { format!("掌握度已达 {}%", mastery as i32) },
                target_id: Some(path.id.clone()),
                target_type: "learning_path".to_string(),
                priority: 2,
                learning_path_id: Some(path.id.clone()),
                learning_path_name: Some(path.title.clone()),
            });
        }
    }

    // Recommendation 3: Re-engage stale paths
    for path in &paths {
        let modules = repo.get_modules_by_path_id(&path.id).map_err(|e| e.to_string())?;
        let mut all_tasks = Vec::new();
        for module in &modules {
            let tasks = repo.get_tasks_by_module_id(&module.id).map_err(|e| e.to_string())?;
            all_tasks.extend(tasks);
        }
        let stale = all_tasks.iter().filter(|t| {
            t.status == "in_progress" || t.status == "not_started"
        }).count();

        if stale > 0 && stale == all_tasks.len() {
            if let Some(first) = all_tasks.first() {
                let desc = if crate::locale::is_english(&locale) {
                    format!("You haven't started \"{}\" yet. Try the first task to get going!", path.title)
                } else {
                    format!("你还没开始「{}」，试着完成第一个任务吧！", path.title)
                };
                recommendations.push(AdaptiveRecommendation {
                    recommendation_type: "reengage".to_string(),
                    title: if crate::locale::is_english(&locale) { "Start a New Path".to_string() } else { "开始新路径".to_string() },
                    description: desc,
                    reason: if crate::locale::is_english(&locale) { "Learning path not started".to_string() } else { "学习路径尚未开始".to_string() },
                    target_id: Some(first.id.clone()),
                    target_type: "practice_task".to_string(),
                    priority: 3,
                    learning_path_id: Some(path.id.clone()),
                    learning_path_name: Some(path.title.clone()),
                });
            }
        }
    }

    Ok(recommendations)
}

// ========== Exam ==========

#[tauri::command]
pub async fn create_exam(
    state: State<'_, AppState>,
    goal_id: String,
    question_count: Option<i32>,
    question_config: Option<String>,
) -> Result<ExamDetail, String> {
    let repo = Repository::new(state.db.clone());
    let n = question_count.unwrap_or(20).clamp(10, 30);
    let locale = crate::locale::resolve_locale(&state.db);

    // Compute next version number for this goal
    let next_version = repo.get_exams_for_goal(&goal_id)
        .map(|exams| exams.iter().map(|e| e.version).max().unwrap_or(0) + 1)
        .unwrap_or(1);
    let next_version = next_version as i32;

    // Get goal's linked wiki pages
    let links = repo.get_goal_wiki_links(&goal_id).map_err(|e| e.to_string())?;
    if links.is_empty() {
        return Err("该目标下没有关联知识点，无法生成考试".to_string());
    }

    // Collect wiki page content
    let mut pages_content = String::new();
    let mut valid_ids = Vec::new();
    for link in &links {
        if let Ok(Some(page)) = repo.get_wiki_page_by_id(&link.wiki_page_id) {
            let truncated: String = page.body_markdown.chars().take(2000).collect();
            pages_content.push_str(&format!(
                "=== {} (id: {}) ===\n{}\n\n",
                page.title, page.id, truncated,
            ));
            valid_ids.push(page.id.clone());
        }
    }

    if valid_ids.is_empty() {
        return Err("关联的知识点内容为空".to_string());
    }

    // Limit content to avoid token overflow
    if pages_content.len() > 8000 {
        pages_content = pages_content.chars().take(8000).collect::<String>() + "\n\n[内容已截断]";
    }

    // Calculate question type distribution: choice ~50%, judgment ~20%, essay ~30%
    let choice_count = (n as f64 * 0.5).round() as i32;
    let judgment_count = (n as f64 * 0.2).round() as i32;
    let essay_count = n - choice_count - judgment_count;

    let system_prompt = if crate::locale::is_english(&locale) {
        format!(
            "You are an exam generator. Create a challenging exam with {} questions based on the knowledge content. \
             Question distribution:\n\
             - {} choice questions (4 options, one correct, include tricky distractors)\n\
             - {} judgment questions (true/false with reasoning required)\n\
             - {} essay questions (open-ended, require explanation)\n\n\
             Requirements:\n\
             - Questions should test understanding, not just memorization\n\
             - Use cross-concept relationships, reverse questioning, application scenarios\n\
             - Each question must specify source_page_id from the content\n\n\
             Return JSON: {{\"title\":\"Exam title\",\"questions\":[{{\"id\":1,\"stem\":\"question\",\"question_type\":\"choice\",\"options\":[\"A\",\"B\",\"C\",\"D\"],\"correct_answer\":\"B\",\"explanation\":\"why\",\"source_page_id\":\"...\"}}]}}\n\
             For judgment: options=[\"对\",\"错\"], correct_answer=\"对\" or \"错\"\n\
             For essay: options=[], correct_answer=\"reference answer\"",
            n, choice_count, judgment_count, essay_count
        )
    } else {
        format!(
            "你是一个考试出题助手。根据以下知识内容生成一份有难度的考试，共 {} 道题。\n\
             题型分配：\n\
             - 选择题 {} 道（4个选项，1个正确，干扰项要有迷惑性）\n\
             - 判断题 {} 道（对/错，需要说明理由）\n\
             - 论述题 {} 道（开放性问题，需要解释说明）\n\n\
             出题要求：\n\
             - 不要直接从原文出题，要变换角度\n\
             - 使用跨知识点关联、反向提问、应用场景题\n\
             - 每道题必须标注 source_page_id（来自内容中的 id）\n\n\
             返回 JSON: {{\"title\":\"考试标题\",\"questions\":[{{\"id\":1,\"stem\":\"题干\",\"question_type\":\"choice\",\"options\":[\"A\",\"B\",\"C\",\"D\"],\"correct_answer\":\"B\",\"explanation\":\"解析\",\"source_page_id\":\"...\"}}]}}\n\
             判断题: options=[\"对\",\"错\"], correct_answer=\"对\" 或 \"错\"\n\
             论述题: options=[], correct_answer=\"参考答案\"",
            n, choice_count, judgment_count, essay_count
        )
    };

    let raw = crate::ai::wiki_engine::call_ai_pub(
        state.db.clone(),
        &system_prompt,
        &pages_content,
        4096,
    )
    .await?;

    let json = crate::ai::wiki_engine::parse_ai_json_pub(&raw)?;

    let title = json.get("title").and_then(|v| v.as_str()).unwrap_or("考试").to_string();
    let questions_raw = json.get("questions").and_then(|v| v.as_array())
        .ok_or("AI 返回格式错误：缺少 questions 数组")?;

    // Create exam record
    let now = chrono::Utc::now().to_rfc3339();
    let exam_id = uuid::Uuid::new_v4().to_string();
    let exam = Exam {
        id: exam_id.clone(),
        goal_id: goal_id.clone(),
        title: Some(title),
        total_questions: questions_raw.len() as i32,
        score: None,
        grade: None,
        status: "in_progress".to_string(),
        started_at: now.clone(),
        completed_at: None,
        diagnosis_json: None,
        created_at: now.clone(),
        version: next_version,
        question_config,
    };
    repo.save_exam(&exam).map_err(|e| e.to_string())?;

    // Save questions
    let mut questions = Vec::new();
    for (i, q_raw) in questions_raw.iter().enumerate() {
        let q_type = q_raw.get("question_type").and_then(|v| v.as_str()).unwrap_or("choice").to_string();
        let source_page_id = q_raw.get("source_page_id").and_then(|v| v.as_str())
            .unwrap_or(valid_ids.first().map(|s| s.as_str()).unwrap_or(""))
            .to_string();
        let correct_answer = q_raw.get("correct_answer").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let q_id = uuid::Uuid::new_v4().to_string();
        let eq = ExamQuestion {
            id: q_id,
            exam_id: exam_id.clone(),
            wiki_page_id: source_page_id,
            question_type: q_type,
            question_json: serde_json::to_string(q_raw).unwrap_or_default(),
            user_answer: None,
            correct_answer: Some(correct_answer),
            is_correct: None,
            score: None,
            ai_feedback: None,
            sort_order: i as i32,
            answered_at: None,
        };
        repo.save_exam_question(&eq).map_err(|e| e.to_string())?;
        questions.push(eq);
    }

    Ok(ExamDetail { exam, questions })
}

#[tauri::command]
pub fn get_exam(
    state: State<'_, AppState>,
    exam_id: String,
) -> Result<ExamDetail, String> {
    let repo = Repository::new(state.db.clone());
    let exam = repo.get_exam_by_id(&exam_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Exam not found: {exam_id}"))?;
    let questions = repo.get_exam_questions(&exam_id).map_err(|e| e.to_string())?;
    Ok(ExamDetail { exam, questions })
}

#[tauri::command]
pub fn submit_exam_answer(
    state: State<'_, AppState>,
    question_id: String,
    answer: String,
) -> Result<ExamQuestion, String> {
    let repo = Repository::new(state.db.clone());

    // Get the question to check answer
    let question = {
        let conn = state.db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;
        let mut stmt = conn.prepare(
            "SELECT id, exam_id, wiki_page_id, question_type, question_json, user_answer, correct_answer, is_correct, score, ai_feedback, sort_order, answered_at FROM exam_questions WHERE id = ?1"
        ).map_err(|e| e.to_string())?;
        let mut rows = stmt.query_map(params![question_id], |row| {
            Ok(ExamQuestion {
                id: row.get(0)?,
                exam_id: row.get(1)?,
                wiki_page_id: row.get(2)?,
                question_type: row.get(3)?,
                question_json: row.get(4)?,
                user_answer: row.get(5)?,
                correct_answer: row.get(6)?,
                is_correct: row.get::<_, Option<i32>>(7)?.map(|v| v != 0),
                score: row.get(8)?,
                ai_feedback: row.get(9)?,
                sort_order: row.get(10)?,
                answered_at: row.get(11)?,
            })
        }).map_err(|e| e.to_string())?;
        match rows.next() {
            Some(Ok(q)) => q,
            _ => return Err(format!("Question not found: {question_id}")),
        }
    };

    // For choice/judgment: auto-grade by comparing with correct_answer
    let (is_correct, score) = match question.question_type.as_str() {
        "choice" | "judgment" => {
            let correct = question.correct_answer.as_deref().unwrap_or("");
            let matched = answer.trim() == correct.trim();
            (Some(matched), Some(if matched { 1.0 } else { 0.0 }))
        }
        "essay" => {
            // Essay questions will be graded later by AI in complete_exam
            (None, None)
        }
        _ => (None, None),
    };

    repo.update_exam_question_answer(&question_id, &answer, is_correct, score, None)
        .map_err(|e| e.to_string())?;

    // Return updated question
    let mut updated = question;
    updated.user_answer = Some(answer);
    updated.is_correct = is_correct;
    updated.score = score;
    updated.answered_at = Some(chrono::Utc::now().to_rfc3339());
    Ok(updated)
}

#[tauri::command]
pub async fn complete_exam(
    state: State<'_, AppState>,
    exam_id: String,
) -> Result<ExamDetail, String> {
    let repo = Repository::new(state.db.clone());
    let mut exam = repo.get_exam_by_id(&exam_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Exam not found: {exam_id}"))?;
    let questions = repo.get_exam_questions(&exam_id).map_err(|e| e.to_string())?;

    // Grade essay questions via AI
    let locale = crate::locale::resolve_locale(&state.db);
    for q in &questions {
        if q.question_type == "essay" && q.user_answer.is_some() && q.score.is_none() {
            let user_ans = q.user_answer.as_deref().unwrap_or("");
            let correct_ref = q.correct_answer.as_deref().unwrap_or("");
            let q_json: serde_json::Value = serde_json::from_str(&q.question_json).unwrap_or_default();
            let stem = q_json.get("stem").and_then(|v| v.as_str()).unwrap_or("");

            let grading_prompt = if crate::locale::is_english(&locale) {
                "You are an exam grader. Grade the student's essay answer on a scale of 0-1 (0=completely wrong, 1=perfect). Return JSON: {\"score\":0.7,\"feedback\":\"brief feedback\"}".to_string()
            } else {
                "你是一个考试评分助手。对学生的论述题答案打分（0-1分，0=完全错误，1=完美）。返回 JSON: {\"score\":0.7,\"feedback\":\"简短评语\"}".to_string()
            };

            let content = format!("题目：{}\n参考答案：{}\n学生答案：{}", stem, correct_ref, user_ans);

            if let Ok(raw) = crate::ai::wiki_engine::call_ai_pub(
                state.db.clone(), &grading_prompt, &content, 512
            ).await {
                if let Ok(grade_json) = crate::ai::wiki_engine::parse_ai_json_pub(&raw) {
                    let essay_score = grade_json.get("score").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let feedback = grade_json.get("feedback").and_then(|v| v.as_str()).unwrap_or("");
                    let is_correct = essay_score >= 0.6;
                    let _ = repo.update_exam_question_answer(&q.id, user_ans, Some(is_correct), Some(essay_score), Some(feedback));
                }
            }
        }
    }

    // Calculate total score
    let graded_questions = repo.get_exam_questions(&exam_id).map_err(|e| e.to_string())?;
    let total = graded_questions.len() as f64;
    let total_score: f64 = graded_questions.iter()
        .filter_map(|q| q.score)
        .sum();
    let percentage = if total > 0.0 { (total_score / total) * 100.0 } else { 0.0 };

    let grade = match percentage as i32 {
        90..=100 => "A",
        75..=89 => "B",
        60..=74 => "C",
        _ => "D",
    };

    // Generate diagnosis
    let weak_pages: Vec<String> = graded_questions.iter()
        .filter(|q| q.is_correct == Some(false))
        .map(|q| q.wiki_page_id.clone())
        .collect();
    let diagnosis = serde_json::json!({
        "weak_wiki_pages": weak_pages,
        "total_correct": graded_questions.iter().filter(|q| q.is_correct == Some(true)).count(),
        "total_wrong": graded_questions.iter().filter(|q| q.is_correct == Some(false)).count(),
    });

    exam.score = Some(percentage);
    exam.grade = Some(grade.to_string());
    exam.status = "completed".to_string();
    exam.completed_at = Some(chrono::Utc::now().to_rfc3339());
    exam.diagnosis_json = Some(serde_json::to_string(&diagnosis).unwrap_or_default());

    repo.update_exam(&exam).map_err(|e| e.to_string())?;

    Ok(ExamDetail { exam, questions: graded_questions })
}

#[tauri::command]
pub fn get_exam_history(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<Vec<ExamSummary>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_exams_by_goal(&goal_id).map_err(|e| e.to_string())
}

// ========== Learning Mode (E-6-6) ==========

#[tauri::command]
pub fn get_learning_content(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<serde_json::Value, String> {
    let repo = Repository::new(state.db.clone());
    let page = repo.get_wiki_page_by_id(&wiki_page_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Wiki page not found: {wiki_page_id}"))?;

    let first_sentence = page.body_markdown
        .split(['.', '。', '\n'])
        .find(|s| !s.trim().is_empty())
        .unwrap_or(&page.body_markdown)
        .trim()
        .to_string();

    Ok(serde_json::json!({
        "title": page.title,
        "concept": format!("{} — {}", page.title, first_sentence),
        "detail": page.body_markdown,
        "extend": format!("标签: {}\n总结: {}", page.tags.unwrap_or_default(), page.summary.unwrap_or_default()),
    }))
}

#[tauri::command]
pub fn mark_as_learned(
    state: State<'_, AppState>,
    goal_id: String,
    wiki_page_id: String,
) -> Result<(), String> {
    // Mark goal links as seen and create/ensure review schedule
    let repo = Repository::new(state.db.clone());
    let _ = repo.mark_goal_wiki_links_seen(&goal_id);
    let existing = repo.get_review_schedule(&wiki_page_id).map_err(|e| e.to_string())?;
    if existing.is_none() {
        repo.create_review_schedule(&wiki_page_id).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn generate_instant_quiz(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<Vec<QuizQuestion>, String> {
    generate_quiz_questions(state, wiki_page_id, Some(2)).await
}

#[tauri::command]
pub fn get_wiki_exam_history(
    state: State<'_, AppState>,
    wiki_page_id: String,
) -> Result<Vec<Exam>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_exams_for_wiki(&wiki_page_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_goal_exams(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<Vec<Exam>, String> {
    let repo = Repository::new(state.db.clone());
    repo.get_exams_for_goal(&goal_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn check_mastery_flags(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<bool, String> {
    let repo = Repository::new(state.db.clone());
    repo.has_unresolved_mastery_flags(&goal_id).map_err(|e| e.to_string())
}

