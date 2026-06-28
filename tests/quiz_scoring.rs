//! Integration tests for quiz pass threshold (75% of questions correct).
//!
//! LEARN: presented questions shuffle choices; scoring uses the shuffled correct index.

use rust_quest::game::quiz::{QuizQuestion, score_presented};

#[test]
fn pass_at_seventy_five_percent() {
    let q = QuizQuestion::new("?", &["a", "b", "c", "d"], 1, "h", "e");
    let presented: Vec<_> = (0..4).map(|i| q.present("errors", i)).collect();
    let answers: Vec<_> = presented.iter().map(|p| p.correct).collect();
    assert!(score_presented(&presented, &answers));
    let fail = vec![0; 4];
    assert!(!score_presented(&presented, &fail));
}
