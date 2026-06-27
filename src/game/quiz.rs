//! Shared types for quiz questions used by every quest.
//!
//! Read this when learning how challenges are defined, or when adding new quests.

/// One multiple-choice question in a quest challenge.
#[derive(Debug, Clone, Copy)]
pub struct QuizQuestion {
    pub prompt: &'static str,
    pub choices: &'static [&'static str],
    pub correct: usize,
    pub hint: &'static str,
    pub explanation: &'static str,
    /// GAME: if true, picking this wrong answer blocks the NoPanic achievement.
    pub is_bad_unwrap_choice: bool,
}

impl QuizQuestion {
    pub const fn new(
        prompt: &'static str,
        choices: &'static [&'static str],
        correct: usize,
        hint: &'static str,
        explanation: &'static str,
    ) -> Self {
        Self {
            prompt,
            choices,
            correct,
            hint,
            explanation,
            is_bad_unwrap_choice: false,
        }
    }
}

/// Returns true when at least 75% of answers are correct.
///
/// # Example
///
/// ```
/// use rust_quest::game::quiz::{score_answers, QuizQuestion};
///
/// let q = QuizQuestion::new("2+2?", &["3", "4", "5", "6"], 1, "hint", "exp");
/// let questions = [q, q, q, q];
/// let answers = [1, 1, 1, 0];
/// assert!(score_answers(&questions, &answers));
/// ```
pub fn score_answers(questions: &[QuizQuestion], answers: &[usize]) -> bool {
    if questions.len() != answers.len() || questions.is_empty() {
        return false;
    }
    let correct = questions
        .iter()
        .zip(answers.iter())
        .filter(|(q, a)| **a == q.correct)
        .count();
    let needed = (questions.len() * 3).div_ceil(4);
    correct >= needed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_of_four_passes() {
        let q = QuizQuestion::new("?", &["a", "b", "c", "d"], 1, "h", "e");
        let qs = [q, q, q, q];
        assert!(score_answers(&qs, &[1, 1, 1, 0]));
        assert!(!score_answers(&qs, &[0, 0, 1, 0]));
    }
}
