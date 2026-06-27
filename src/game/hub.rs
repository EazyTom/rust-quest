//! Hub menu, quest flow, quizzes, and resource browser.
//!
//! Uses dialoguer for menus; quest map uses crossterm (see ui/map.rs).

use std::io;

use colored::Colorize;
use dialoguer::{Confirm, Input, Select};

use crate::game::progress;
use crate::game::quiz::{QuizQuestion, score_answers};
use crate::game::state::{GameState, QuestStep, StepResult, today_string};
use crate::game::ui::{copy, retro, run_quest_map};
use crate::game::xp::{self, Rank};
use crate::resources::links::open_url;
use crate::topics::registry::{self, Quest};

fn dialoguer_err(e: dialoguer::Error) -> io::Error {
    io::Error::other(e.to_string())
}

/// Esc / q returns `None` — always treat as Back.
fn select_menu(prompt: &str, items: &[&str], default: usize) -> io::Result<Option<usize>> {
    Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact_opt()
        .map_err(dialoguer_err)
}

fn confirm_menu(prompt: &str, default: bool) -> io::Result<Option<bool>> {
    Confirm::new()
        .with_prompt(prompt)
        .default(default)
        .interact_opt()
        .map_err(dialoguer_err)
}

fn quest_map_session(state: &mut GameState) -> io::Result<()> {
    // GAME: map → quest → map until Esc from the map returns to hub.
    while let Some(quest_id) = run_quest_map(state)? {
        run_quest(state, quest_id)?;
    }
    Ok(())
}

pub fn run_hub(state: &mut GameState) -> io::Result<bool> {
    if state.player_name.is_empty() {
        let name: String = Input::new()
            .with_prompt("Welcome to Rust Quest! Your name")
            .default("Ayush".into())
            .interact_text()
            .map_err(dialoguer_err)?;
        state.player_name = name;
    }

    loop {
        print_hub(state);
        println!("{}", retro::section_header("Main Menu"));
        let choices = &[
            "Quest Map",
            "Profile",
            "Resources",
            "Sandbox",
            "Unlock All (practice)",
            "Reset progress",
            "Quit",
        ];
        let sel = select_menu("Choose (↑/↓, Enter, Esc back)", choices, 0)?;
        let Some(sel) = sel else {
            continue;
        };

        match sel {
            0 => quest_map_session(state)?,
            1 => show_profile(state),
            2 => resource_menu()?,
            3 => sandbox_menu()?,
            4 => {
                state.practice_unlock_all = true;
                println!("{}", retro::success("Practice mode: all quests unlocked."));
            }
            5 => {
                if confirm_menu("Reset all progress?", false)?.unwrap_or(false) {
                    let name = state.player_name.clone();
                    state.reset();
                    state.player_name = name;
                    let _ = progress::save_progress(state);
                    println!("{}", retro::success("Progress reset."));
                }
            }
            6 => return Ok(true),
            _ => {}
        }
        let _ = progress::save_progress(state);
        println!("\n{}\n", copy::session_quote().bright_white());
    }
}

fn print_hub(state: &GameState) {
    let rank = state.rank();
    let bar = xp::xp_bar(state.xp, 12);
    println!("\n{}", retro::title_banner());
    println!(
        "{}",
        retro::box_line(&format!(
            "Player: {}   Rank: {} {}",
            state.player_name,
            rank.emoji(),
            rank.title()
        ))
    );
    println!(
        "{}",
        retro::box_line(&format!(
            "XP {} {}   Streak: {} {}d",
            bar, state.xp, "🔥", state.streak_days
        ))
    );
    println!("{}\n", retro::box_bottom());
}

fn show_profile(state: &GameState) {
    println!("{}", retro::section_header("👤 Profile"));
    println!("Name: {}", state.player_name);
    println!("XP: {} / {}", state.xp, xp::MAX_XP);
    println!("Quests completed: {}/14", state.completed_quests.len());
    println!("Achievements:");
    if state.achievements.is_empty() {
        println!("  (none yet — play quests!)");
    } else {
        for id in &state.achievements {
            if let Some((emoji, name)) = crate::game::achievements::display_name(id) {
                println!("  {} {}", emoji, name);
            }
        }
    }
}

fn sandbox_menu() -> io::Result<()> {
    let quests: Vec<&Quest> = registry::all().iter().collect();
    let labels: Vec<String> = quests
        .iter()
        .map(|q| format!("{} {} {}", q.emoji, q.title, "(demo only)"))
        .collect();
    let labels_ref: Vec<&str> = labels.iter().map(String::as_str).collect();
    let idx = select_menu("Sandbox — replay demo", &labels_ref, 0)?;
    let Some(idx) = idx else {
        return Ok(());
    };
    let q = quests[idx];
    println!("\n{}\n", (q.demo)());
    println!("Memory note: {}", q.memory_note);
    Ok(())
}

fn resource_menu() -> io::Result<()> {
    let quests = registry::all();
    let labels: Vec<String> = quests
        .iter()
        .map(|q| format!("{} {}", q.emoji, q.title))
        .collect();
    let labels_ref: Vec<&str> = labels.iter().map(String::as_str).collect();
    let idx = select_menu("Open resources for quest", &labels_ref, 0)?;
    let Some(idx) = idx else {
        return Ok(());
    };
    open_links_menu(&quests[idx])?;
    Ok(())
}

fn open_links_menu(quest: &Quest) -> io::Result<()> {
    let items = &[
        "The Rust Book",
        "Rust by Example",
        "std docs (if any)",
        "YouTube",
    ];
    loop {
        let sel = select_menu(&format!("Resources: {}", quest.title), items, 0)?;
        let Some(sel) = sel else {
            break;
        };
        match sel {
            0 => open_url(quest.links.book),
            1 => open_url(quest.links.rust_by_example),
            2 => {
                if let Some(u) = quest.links.std_docs {
                    open_url(u);
                } else {
                    println!("No std doc link for this quest.");
                }
            }
            3 => {
                if let Some(u) = quest.links.youtube.first() {
                    open_url(u);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn run_quest(state: &mut GameState, quest_id: &str) -> io::Result<()> {
    let Some(quest) = registry::find(quest_id) else {
        return Ok(());
    };
    if !state.is_unlocked(quest_id) {
        println!(
            "{}",
            retro::failure("Quest locked — complete the previous quest first.")
        );
        return Ok(());
    }

    let steps = &[
        "Learn — read the demo walkthrough (+XP once)",
        "Challenge — quiz to unlock the next quest",
        "Explore resources — open book & video links",
    ];
    loop {
        println!(
            "{}",
            retro::section_header(&format!("{} {}", quest.emoji, quest.title))
        );
        let step = select_menu("Quest step (↑/↓, Enter, Esc back to map)", steps, 0)?;
        let Some(step) = step else {
            break;
        };
        match step {
            0 => run_learn(state, quest),
            1 => run_challenge(state, quest)?,
            2 => open_links_menu(&quest)?,
            _ => {}
        }
    }
    Ok(())
}

fn run_learn(state: &mut GameState, quest: Quest) {
    println!("\n{}\n", (quest.demo)());
    println!("{}", "Memory safety:".bright_yellow().bold());
    println!("{}\n", quest.memory_note);
    let today = today_string();
    match state.complete_step(quest.id, QuestStep::Learn, &today) {
        StepResult::XpGained { amount, .. } => {
            println!("{}", retro::success(&format!("+{amount} XP")));
        }
        StepResult::RankUp { rank } => println!("{}", copy::rank_up(rank).green()),
        StepResult::AlreadyDone => println!("Learn step already completed (no extra XP)."),
        StepResult::QuestCompleted { .. } => {}
    }
}

fn run_challenge(state: &mut GameState, quest: Quest) -> io::Result<()> {
    if state.step_done(quest.id, QuestStep::Challenge) {
        println!("Challenge already passed — no extra XP.");
        return Ok(());
    }

    state.ownership_passed_first_try = quest.id == "ownership";
    state.errors_challenge_picked_unwrap = false;

    let mut questions: Vec<QuizQuestion> = quest.questions.to_vec();
    questions.push(quest.boss);

    let mut answers = Vec::new();
    let mut any_wrong = false;
    for (i, q) in questions.iter().enumerate() {
        let label = if i < 3 {
            format!("Question {}", i + 1)
        } else {
            "Boss question".to_string()
        };
        println!("\n{}", label.bright_cyan());
        println!("{}", q.prompt);
        let idx = select_menu(q.prompt, q.choices, q.correct)?;
        let Some(idx) = idx else {
            println!("{}", "Challenge paused — Esc back to quest steps.".dimmed());
            return Ok(());
        };
        if idx == 0 && (q.choices[0].contains("unwrap()") || q.is_bad_unwrap_choice) {
            state.errors_challenge_picked_unwrap = true;
        }
        if idx != q.correct {
            any_wrong = true;
            println!("Hint: {}", q.hint.yellow());
            println!("{}", q.explanation.dimmed());
        }
        answers.push(idx);
    }

    if any_wrong && confirm_menu("Open Rust Book link?", true)?.unwrap_or(false) {
        open_url(quest.links.book);
    }

    if score_answers(&questions, &answers) {
        state.ownership_passed_first_try =
            state.ownership_passed_first_try && quest.id == "ownership";
        let rank_before = state.rank();
        let today = today_string();
        let result = state.complete_step(quest.id, QuestStep::Challenge, &today);
        let rank_after = state.rank();
        println!("{}", retro::success(copy::quiz_pass()));
        match result {
            StepResult::XpGained { amount, .. } => {
                println!("{}", retro::success(&format!("+{amount} XP")));
            }
            StepResult::QuestCompleted { .. } => {
                println!("{}", retro::success("Quest complete!"));
            }
            _ => {}
        }
        if rank_after != rank_before {
            println!("{}", copy::rank_up(rank_after).green().bold());
        }
        if rank_after == Rank::Champion {
            println!(
                "{}",
                "👑 You are Rust Quest Champion!".bright_yellow().bold()
            );
        }
    } else {
        println!("{}", retro::failure(copy::quiz_fail()));
        state.ownership_passed_first_try = false;
    }
    Ok(())
}
