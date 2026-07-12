//! Shared types for quiz questions used by every quest.
//!
//! Read this when learning how challenges are defined, or when adding new quests.

use std::hash::{Hash, Hasher};

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

/// Question with choices rotated so the cursor default (index 0) is not always correct.
#[derive(Debug, Clone)]
pub struct PresentedQuestion {
    pub prompt: &'static str,
    pub choices: Vec<&'static str>,
    pub correct: usize,
    pub hint: &'static str,
    pub explanation: &'static str,
    /// Display indices that pick an unwrap() answer (for achievements).
    bad_unwrap_indices: Vec<usize>,
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

    /// Rotate choices per quest/question so correct answers land on different keys.
    pub fn present(&self, quest_id: &str, question_index: u32) -> PresentedQuestion {
        let n = self.choices.len().max(1);
        let offset = shuffle_offset(quest_id, question_index, self.prompt) % n;
        let mut choices = Vec::with_capacity(n);
        let mut correct = 0;
        let mut bad_unwrap_indices = Vec::new();
        for display_idx in 0..n {
            let src = (display_idx + offset) % n;
            choices.push(self.choices[src]);
            if self.choices[src].contains("unwrap()") {
                bad_unwrap_indices.push(display_idx);
            }
            if src == self.correct {
                correct = display_idx;
            }
        }
        PresentedQuestion {
            prompt: self.prompt,
            choices,
            correct,
            hint: self.hint,
            explanation: self.explanation,
            bad_unwrap_indices,
        }
    }
}

impl PresentedQuestion {
    pub fn choice_labels(&self) -> Vec<&str> {
        self.choices.clone()
    }

    /// GAME: tracks NoPanic achievement — true when player picks a shuffled unwrap() option.
    pub fn is_bad_unwrap_pick(&self, selected: usize) -> bool {
        self.bad_unwrap_indices.contains(&selected)
    }
}

fn shuffle_offset(quest_id: &str, question_index: u32, prompt: &str) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    quest_id.hash(&mut hasher);
    question_index.hash(&mut hasher);
    prompt.hash(&mut hasher);
    hasher.finish() as usize
}

/// Returns true when at least 75% of answers are correct.
///
/// # Example
///
/// ```
/// use rust_quest::game::quiz::{QuizQuestion, score_presented};
///
/// let q = QuizQuestion::new("2+2?", &["3", "4", "5", "6"], 1, "hint", "exp");
/// let p = q.present("cargo", 0);
/// let answers = [p.correct];
/// assert!(score_presented(&[p], &answers));
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

pub fn score_presented(questions: &[PresentedQuestion], answers: &[usize]) -> bool {
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
    fn bad_unwrap_pick_tracks_unwrap_choice() {
        let mut q = QuizQuestion::new("pick?", &["unwrap()", "match", "?", "panic"], 1, "h", "e");
        q.is_bad_unwrap_choice = true;
        let p = q.present("errors", 0);
        let unwrap_idx = p
            .choices
            .iter()
            .position(|c| c.contains("unwrap()"))
            .expect("unwrap choice");
        assert!(p.is_bad_unwrap_pick(unwrap_idx));
    }

    #[test]
    fn three_of_four_passes() {
        let q = QuizQuestion::new("?", &["a", "b", "c", "d"], 1, "h", "e");
        let qs = [q, q, q, q];
        assert!(score_answers(&qs, &[1, 1, 1, 0]));
        assert!(!score_answers(&qs, &[0, 0, 1, 0]));
    }

    #[test]
    fn present_moves_correct_away_from_zero_sometimes() {
        let q = QuizQuestion::new("?", &["a", "b", "c", "d"], 1, "h", "e");
        let mut saw_non_zero = false;
        for i in 0..20 {
            let p = q.present("cargo", i);
            if p.correct != 0 {
                saw_non_zero = true;
            }
            assert!(p.correct < p.choices.len());
        }
        assert!(saw_non_zero);
    }

    #[test]
    fn presented_scoring_uses_shuffled_index() {
        let q = QuizQuestion::new("?", &["a", "b", "c", "d"], 1, "h", "e");
        let p = q.present("types", 2);
        let answers = vec![p.correct];
        assert!(score_presented(&[p], &answers));
    }
}
