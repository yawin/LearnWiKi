//! Keyword overlap fallback used when AI scoring is unavailable.
//!
//! Tokenization rule:
//! - Split by ASCII whitespace and ASCII punctuation
//! - Lowercase all ASCII letters
//! - Drop English stopwords (a/the/and/...)
//! - For runs of CJK characters: emit 2-character bigrams (sliding window)
//! - Drop single-character tokens

use std::collections::HashSet;

const STOPWORDS: &[&str] = &[
    "a", "an", "the", "and", "or", "of", "to", "in", "on", "for",
    "with", "is", "are", "be", "by", "as", "at", "it", "this", "that",
    "i", "you", "we", "they", "he", "she",
];

pub fn tokenize(text: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    let mut buf: Vec<char> = Vec::new();
    let mut cjk_buf: Vec<char> = Vec::new();

    let flush_ascii = |buf: &mut Vec<char>, out: &mut HashSet<String>| {
        if buf.is_empty() {
            return;
        }
        let token: String = buf.iter().collect::<String>().to_lowercase();
        buf.clear();
        if token.chars().count() <= 1 {
            return;
        }
        if STOPWORDS.contains(&token.as_str()) {
            return;
        }
        out.insert(token);
    };

    let flush_cjk = |buf: &mut Vec<char>, out: &mut HashSet<String>| {
        if buf.len() < 2 {
            buf.clear();
            return;
        }
        for window in buf.windows(2) {
            let bigram: String = window.iter().collect();
            out.insert(bigram);
        }
        buf.clear();
    };

    for ch in text.chars() {
        let is_cjk = is_cjk(ch);
        let is_ascii_alnum = ch.is_ascii_alphanumeric();
        if is_cjk {
            flush_ascii(&mut buf, &mut out);
            cjk_buf.push(ch);
        } else if is_ascii_alnum {
            flush_cjk(&mut cjk_buf, &mut out);
            buf.push(ch);
        } else {
            flush_ascii(&mut buf, &mut out);
            flush_cjk(&mut cjk_buf, &mut out);
        }
    }
    flush_ascii(&mut buf, &mut out);
    flush_cjk(&mut cjk_buf, &mut out);
    out
}

fn is_cjk(ch: char) -> bool {
    matches!(ch as u32,
        0x4E00..=0x9FFF  // CJK Unified
        | 0x3400..=0x4DBF  // Extension A
        | 0x3040..=0x30FF  // Hiragana/Katakana
    )
}

/// Returns overlap ratio between `wiki_terms` and `goal_terms`,
/// computed as |intersection| / |goal_terms|.
/// Returns 0.0 if `goal_terms` is empty.
pub fn overlap_ratio(wiki_terms: &HashSet<String>, goal_terms: &HashSet<String>) -> f64 {
    if goal_terms.is_empty() {
        return 0.0;
    }
    let intersection = wiki_terms.intersection(goal_terms).count();
    intersection as f64 / goal_terms.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_english_lowercases_and_drops_stopwords() {
        let got = tokenize("The Rust Ownership Model");
        assert!(got.contains("rust"));
        assert!(got.contains("ownership"));
        assert!(got.contains("model"));
        assert!(!got.contains("the"), "stopword 'the' should be dropped");
    }

    #[test]
    fn tokenize_drops_single_char_ascii() {
        let got = tokenize("a b cd");
        assert!(!got.contains("a"));
        assert!(!got.contains("b"));
        assert!(got.contains("cd"));
    }

    #[test]
    fn tokenize_chinese_emits_bigrams() {
        let got = tokenize("所有权");
        // 所有权 -> bigrams: "所有", "有权"
        assert!(got.contains("所有"));
        assert!(got.contains("有权"));
        assert_eq!(got.len(), 2);
    }

    #[test]
    fn tokenize_mixed_chinese_english() {
        let got = tokenize("Rust 所有权 model");
        assert!(got.contains("rust"));
        assert!(got.contains("所有"));
        assert!(got.contains("有权"));
        assert!(got.contains("model"));
    }

    #[test]
    fn overlap_ratio_full_match() {
        let wiki: HashSet<String> = ["rust", "ownership", "borrow"]
            .iter().map(|s| s.to_string()).collect();
        let goal: HashSet<String> = ["rust", "ownership"]
            .iter().map(|s| s.to_string()).collect();
        assert_eq!(overlap_ratio(&wiki, &goal), 1.0);
    }

    #[test]
    fn overlap_ratio_half_match() {
        let wiki: HashSet<String> = ["rust"].iter().map(|s| s.to_string()).collect();
        let goal: HashSet<String> = ["rust", "ownership"]
            .iter().map(|s| s.to_string()).collect();
        assert_eq!(overlap_ratio(&wiki, &goal), 0.5);
    }

    #[test]
    fn overlap_ratio_empty_goal_returns_zero() {
        let wiki: HashSet<String> = ["rust"].iter().map(|s| s.to_string()).collect();
        let goal: HashSet<String> = HashSet::new();
        assert_eq!(overlap_ratio(&wiki, &goal), 0.0);
    }
}
