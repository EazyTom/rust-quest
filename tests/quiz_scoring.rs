//! Integration tests for quiz pass threshold (75% of questions correct).
//!
//! LEARN: doc tests in `quiz.rs` and these tests both verify `score_answers`.

use rust_quest::game::quiz::{QuizQuestion, score_answers};

#[test]
fn pass_at_seventy_five_percent() {
    let q = QuizQuestion::new("?", &["a", "b", "c", "d"], 1, "h", "e");
    let qs = [q, q, q, q];
    assert!(score_answers(&qs, &[1, 1, 1, 0]));
    assert!(!score_answers(&qs, &[0, 0, 1, 0]));
}
