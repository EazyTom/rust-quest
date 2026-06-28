//! Hub menu, quest flow, quizzes, and resource browser.
//!
//! Uses dialoguer for menus; quest map uses crossterm (see ui/map.rs).

use std::io;

use colored::Colorize;
use dialoguer::{Confirm, Input, Select};

use crate::game::epic;
use crate::game::narrative;
use crate::game::progress;
use crate::game::audio::{self, MusicHandle, MusicMode};
use crate::game::quiz::{PresentedQuestion, QuizQuestion, score_presented};
use crate::game::state::{GameState, QuestStep, StepResult, today_string};
use crate::game::ui::{copy, input, retro, run_quest_map};
use crate::game::xp::{self, XP_CHALLENGE, XP_DUNGEON_BOSS};
use crate::resources::links::open_url;
use crate::topics::registry::{self, Quest};

fn dialoguer_err(e: dialoguer::Error) -> io::Error {
    io::Error::other(e.to_string())
}

/// Esc cancels dialoguer (`None`). Callers decide: hub menu Esc quits; quest step Esc returns to map.
fn select_menu(prompt: &str, items: &[&str]) -> io::Result<Option<usize>> {
    Select::new()
        .with_prompt(prompt)
        .items(items)
        .default(0)
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

fn quest_map_session(state: &mut GameState, music: &MusicHandle) -> io::Result<()> {
    // GAME: map → auto-chain quests after quiz pass → map when Esc from map or quest menu.
    input::clear_screen_quiet();
    while let Some(first_id) = run_quest_map(state)? {
        let mut quest_id = first_id;
        loop {
            match run_quest(state, quest_id, music)? {
                Some(next_id) => quest_id = next_id,
                None => {
                    // GAME: Esc from quest steps — clear room output before the map redraws.
                    input::clear_screen_quiet();
                    break;
                }
            }
        }
    }
    // GAME: Esc from map — clear before hub redraw so map tiles do not linger above the banner.
    input::clear_screen_quiet();
    input::drain_pending_keys_quiet();
    Ok(())
}

/// GAME: after a quiz pass, offer the Rust Book — default Yes when any answer was wrong.
fn offer_quest_book(state: &mut GameState, quest: &Quest, any_wrong: bool) -> io::Result<()> {
    if confirm_menu(&copy::book_chapter_prompt(quest.title), any_wrong)?.unwrap_or(false) {
        open_url(quest.links.book);
        grant_healing_from_resources(state);
    }
    Ok(())
}

fn grant_healing_potion(state: &mut GameState, message: &str) {
    if state.drink_lore_potion() {
        println!("{}", retro::dungeon_master_says(message));
    }
}

fn grant_healing_from_resources(state: &mut GameState) {
    grant_healing_potion(state, copy::resource_potion());
}

fn print_next_quest_guidance(next: Quest) {
    println!();
    let line = narrative::encounter_for(next)
        .map(|enc| copy::next_quest_guidance(next.emoji, next.title, enc.room_name))
        .unwrap_or_else(|| format!("🧭 Onward to {} {}!", next.emoji, next.title));
    println!("{}", retro::dungeon_master_says(&line));
}

/// GAME: one quiz strike — bright prompt + dialoguer choices; Esc pauses the fight.
fn ask_quiz_question(q: &PresentedQuestion, round_label: &str) -> io::Result<Option<usize>> {
    println!("\n{}", round_label);
    println!("{}", q.prompt.bright_white());
    let labels = q.choice_labels();
    let label_refs: Vec<&str> = labels.iter().copied().collect();
    select_menu("Choose (↑/↓, Enter)", &label_refs)
}

fn print_answer_result(q: &PresentedQuestion, idx: usize, verbose_miss: bool) {
    if idx != q.correct {
        println!("{}", retro::combat_miss());
        if verbose_miss {
            println!("{}", copy::wrong_answer_hint().yellow());
        }
        println!("Hint: {}", q.hint.yellow());
        println!("{}", q.explanation.dimmed());
    } else {
        println!("{}", retro::combat_hit());
    }
}

fn apply_quiz_heart(state: &mut GameState, correct: bool) {
    if correct {
        if state.gain_heart() {
            println!("{}", retro::success(copy::heart_gained()));
        }
    } else {
        state.lose_heart();
        println!("{}", copy::heart_lost(state.hearts).bright_red());
        if state.is_weakened() {
            println!("{}", retro::dungeon_master_says(copy::hearts_depleted()));
        }
    }
}

fn quest_combat_label(
    question_index: usize,
    regular_count: usize,
    enc: Option<&narrative::QuestEncounter>,
) -> String {
    if question_index < regular_count {
        retro::combat_round(question_index + 1, regular_count + 1)
    } else if let Some(e) = enc {
        retro::final_gambit(e.enemy_emoji, e.enemy_name)
    } else {
        retro::final_gambit("💀", "Room Boss")
    }
}

fn print_farewell(state: &GameState) {
    println!();
    println!("{}", retro::box_top("🌙  Campfire Farewell"));
    for line in copy::farewell_lines(&state.player_name) {
        println!("{}", retro::box_line(&line));
    }
    println!("{}\n", retro::box_bottom());
}

pub fn run_hub(state: &mut GameState, music: &MusicHandle) -> io::Result<()> {
    if state.player_name.is_empty() {
        let name: String = Input::new()
            .with_prompt("🎲 Dungeon Master: State thy name, adventurer")
            .default("Ayush".into())
            .interact_text()
            .map_err(dialoguer_err)?;
        state.player_name = name;
        // GAME: Enter after name entry must not auto-pick Quest Map on the first hub menu.
        input::drain_pending_keys_quiet();
    }

    if state.is_champion() && !state.victory_celebrated {
        epic::celebrate_champion(state);
        state.mark_victory_celebrated();
        let _ = progress::save_progress(state);
    }

    loop {
        input::clear_screen_quiet();
        print_hub(state);
        print_hub_adventure_intro(state);
        if state.is_champion() {
            println!(
                "{}",
                "👑🪙💰 Champion — the dungeon is yours to revisit! 🧭📜"
                    .bright_yellow()
                    .bold()
            );
        }
        println!("{}", retro::main_menu_frame());
        let choices = &[
            "🧭 Quest Map — enter the dungeon",
            "📜 Resources — open lore scrolls",
            "🛠️ Sandbox — replay demo runes",
            "📖 Book study guide — gaps & next steps",
            "🔓 Unlock All — practice mode",
            "💾 Reset progress — wipe the slate",
            "🎼 Music — mute or change track",
            "☕ Quit — leave the dungeon",
        ];
        let sel = select_menu("Choose (↑/↓, Enter, Esc to quit)", choices)?;
        let Some(sel) = sel else {
            print_farewell(state);
            return Ok(());
        };

        match sel {
            0 => quest_map_session(state, music)?,
            1 => resource_menu(state)?,
            2 => sandbox_menu()?,
            3 => {
                epic::print_book_gaps_guide();
                grant_healing_from_resources(state);
                if state.is_champion() {
                    println!(
                        "{}",
                        "You beat the game — deep-read each quest’s 📖 book scrolls!"
                            .bright_green()
                    );
                }
            }
            4 => {
                state.practice_unlock_all = true;
                println!("{}", retro::success("Practice mode: all quests unlocked."));
            }
            5 => {
                if confirm_menu("Reset all progress?", false)?.unwrap_or(false) {
                    state.reset();
                    let _ = progress::save_progress(state);
                    println!("{}", retro::success("Progress reset."));
                }
            }
            6 => music_menu(state, music)?,
            7 => {
                print_farewell(state);
                return Ok(());
            }
            _ => {}
        }
        let _ = progress::save_progress(state);
        println!("\n{}\n", retro::dungeon_master_says(copy::session_quote()));
    }
}

fn print_hub_adventure_intro(state: &GameState) {
    if state.is_champion() {
        return;
    }

    let lines: [String; 3] = if state.is_fresh_adventurer() {
        copy::hub_welcome_lines(&state.player_name)
    } else if let Some(quest) = state.next_active_quest() {
        let study_first = !state.step_done(quest.id, QuestStep::Learn);
        if let Some(enc) = narrative::encounter_for(quest) {
            copy::hub_quest_guidance_lines(
                &state.player_name,
                quest.emoji,
                quest.title,
                enc.room_name,
                enc.enemy_emoji,
                enc.enemy_name,
                study_first,
            )
        } else {
            copy::hub_quest_guidance_lines(
                &state.player_name,
                quest.emoji,
                quest.title,
                quest.title,
                quest.emoji,
                "the room's foe",
                study_first,
            )
        }
    } else {
        return;
    };

    println!();
    println!("{}", retro::box_top("🎲  Dungeon Master"));
    for line in &lines {
        println!("{}", retro::dungeon_master_box_line(line));
    }
    println!("{}\n", retro::box_bottom());
}

fn print_hub(state: &GameState) {
    let rank = state.rank();
    let bar = xp::xp_bar(state.xp, 12);
    println!("\n{}", retro::title_banner());
    println!(
        "{}",
        retro::box_line(&format!(
            "Player: 🧙 {} {}",
            state.player_name,
            state.hearts_bar()
        ))
    );
    println!(
        "{}",
        retro::box_line(&format!(
            "Rank: {} {}",
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
    let music_line = audio::status_label(state);
    println!("{}", retro::box_line(&music_line));
    println!(
        "{}",
        retro::box_line(&format!("v{}", crate::version::VERSION))
    );
    println!("{}\n", retro::box_bottom());
}

fn sandbox_menu() -> io::Result<()> {
    let quests: Vec<&Quest> = registry::all().iter().collect();
    let labels: Vec<String> = quests
        .iter()
        .map(|q| format!("{} {} {}", q.emoji, q.title, "(demo only)"))
        .collect();
    let labels_ref: Vec<&str> = labels.iter().map(String::as_str).collect();
    let idx = select_menu("Sandbox — replay demo", &labels_ref)?;
    let Some(idx) = idx else {
        return Ok(());
    };
    let q = quests[idx];
    println!("\n{}\n", (q.demo)());
    println!("Memory note: {}", q.memory_note);
    Ok(())
}

fn music_menu(state: &mut GameState, music: &MusicHandle) -> io::Result<()> {
    loop {
        let tracks = audio::discover_tracks();
        let mute_label = if state.music_muted {
            "Unmute music 🔊"
        } else {
            "Mute music 🔇"
        };
        println!("{}", retro::section_header("🎵 Dungeon Music"));
        println!(
            "{}",
            format!("Music: {}", audio::status_label(state)).bright_cyan()
        );
        if !music.is_available() {
            println!(
                "{}",
                "No audio device detected — music controls are saved but silent here."
                    .dimmed()
            );
        }
        if tracks.is_empty() {
            println!(
                "{}",
                "No .mp3 files in assets/music/ — add tracks and restart."
                    .yellow()
            );
            let _ = select_menu("Back", &["Back"])?;
            break;
        }

        let mut choices: Vec<String> = tracks
            .iter()
            .map(|t| {
                let mark = if state.music_mode == MusicMode::Fixed && state.music_track == t.stem {
                    " ◀"
                } else {
                    ""
                };
                format!("{}{}", t.label, mark)
            })
            .collect();
        let cycle_mark = if state.music_mode == MusicMode::CycleOnQuest {
            " ◀"
        } else {
            ""
        };
        choices.push(format!(
            "Cycle — rotate track on each quest{cycle_mark}"
        ));
        choices.push(mute_label.to_string());
        choices.push("Back".to_string());
        let choice_refs: Vec<&str> = choices.iter().map(String::as_str).collect();
        let sel = select_menu("Choose track or mute (↑/↓, Enter, Esc back)", &choice_refs)?;
        let Some(sel) = sel else {
            break;
        };

        let cycle_idx = tracks.len();
        let mute_idx = tracks.len() + 1;

        if sel < tracks.len() {
            state.music_mode = MusicMode::Fixed;
            let stem = tracks[sel].stem.clone();
            let changed = state.music_track != stem;
            state.music_track = stem.clone();
            state.music_muted = false;
            state.music_playing_stem = stem.clone();
            // GAME: only restart the sink when the pinned track actually changes.
            if changed {
                music.play_stem(&stem);
            }
            println!(
                "{}",
                retro::success(&format!(
                    "Pinned: {} — plays until you change it.",
                    tracks[sel].label
                ))
            );
            let _ = progress::save_progress(state);
        } else if sel == cycle_idx {
            state.music_mode = MusicMode::CycleOnQuest;
            if state.music_playing_stem.is_empty() {
                if let Some(stem) = audio::default_track_stem() {
                    state.music_playing_stem = stem.clone();
                    state.music_last_stem = stem;
                }
            } else {
                state.music_last_stem = state.music_playing_stem.clone();
            }
            if !state.music_muted {
                music.play_stem(&state.music_playing_stem);
            }
            println!(
                "{}",
                retro::success("Cycle on — track changes each quest you enter.")
            );
            let _ = progress::save_progress(state);
        } else if sel == mute_idx {
            state.music_muted = !state.music_muted;
            if state.music_muted {
                music.set_muted();
            } else {
                MusicHandle::apply_session_playback(state, music);
            }
            let msg = if state.music_muted {
                "Music muted — the dungeon falls silent."
            } else {
                "Music restored — the dungeon hums again."
            };
            println!("{}", retro::success(msg));
            let _ = progress::save_progress(state);
        } else {
            break;
        }
    }
    Ok(())
}

fn resource_menu(state: &mut GameState) -> io::Result<()> {
    let quests = registry::all();
    let labels: Vec<String> = quests
        .iter()
        .map(|q| format!("{} {}", q.emoji, q.title))
        .collect();
    let labels_ref: Vec<&str> = labels.iter().map(String::as_str).collect();
    let idx = select_menu("Open resources for quest", &labels_ref)?;
    let Some(idx) = idx else {
        return Ok(());
    };
    open_links_menu(state, &quests[idx])?;
    Ok(())
}

fn open_links_menu(state: &mut GameState, quest: &Quest) -> io::Result<()> {
    let items = &[
        "📖 The Rust Book",
        "📜 Rust by Example",
        "📖 std docs (if any)",
        "📜 YouTube scrolls",
    ];
    loop {
        let sel = select_menu(&format!("Resources: {}", quest.title), items)?;
        let Some(sel) = sel else {
            break;
        };
        match sel {
            0 => {
                open_url(quest.links.book);
                grant_healing_from_resources(state);
            }
            1 => {
                open_url(quest.links.rust_by_example);
                grant_healing_from_resources(state);
            }
            2 => {
                if let Some(u) = quest.links.std_docs {
                    open_url(u);
                    grant_healing_from_resources(state);
                } else {
                    println!("No std doc link for this quest.");
                }
            }
            3 => {
                if let Some(u) = quest.links.youtube.first() {
                    open_url(u);
                    grant_healing_from_resources(state);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn run_quest(
    state: &mut GameState,
    quest_id: &str,
    music: &MusicHandle,
) -> io::Result<Option<&'static str>> {
    let Some(quest) = registry::find(quest_id) else {
        return Ok(None);
    };
    if !state.is_unlocked(quest_id) {
        println!("{}", retro::failure(copy::quest_locked()));
        return Ok(None);
    }

    // GAME: cycle mode advances the track when a quest room opens — pinned fixed mode is a no-op.
    MusicHandle::on_quest_start(state, music);

    narrative::print_room_arrival(quest);

    let enc = narrative::encounter_for(quest);
    let steps = &[
        "💡 Study the runes — Learn (+XP once)",
        "⚔️ Face the foe — quiz encounter",
        "📖 Consult scrolls — book & video links",
    ];
    // GAME: Learn / Challenge / Resources until Esc (map) or quiz pass chains to next quest.
    loop {
        let header = if let Some(e) = enc {
            format!("{} {} · {}", quest.emoji, e.room_name, quest.title)
        } else {
            format!("{} {}", quest.emoji, quest.title)
        };
        println!("{}", retro::section_header(&header));
        let step = select_menu("Quest step (↑/↓, Enter, Esc back to map)", steps)?;
        let Some(step) = step else {
            return Ok(None);
        };
        match step {
            0 => run_learn(state, quest),
            1 => {
                if run_challenge(state, quest)? {
                    if let Some(next) = state.next_active_quest() {
                        print_next_quest_guidance(next);
                        return Ok(Some(next.id));
                    }
                    println!("{}", retro::success(copy::all_quests_cleared()));
                    return Ok(None);
                }
            }
            2 => open_links_menu(state, &quest)?,
            _ => {}
        }
    }
}

fn run_learn(state: &mut GameState, quest: Quest) {
    if let Some(enc) = narrative::encounter_for(quest) {
        println!("\n{}", retro::dungeon_master_says(enc.learn_prompt));
    }
    println!("\n{}\n", (quest.demo)());
    println!("{}", copy::memory_safety_header().bright_yellow().bold());
    println!("{}\n", quest.memory_note.bright_yellow());
    let today = today_string();
    match state.complete_step(quest.id, QuestStep::Learn, &today) {
        StepResult::XpGained { amount, .. } => {
            println!("{}", retro::success(&format!("+{amount} XP — {}", copy::learn_complete())));
        }
        StepResult::AlreadyDone => println!("{}", copy::learn_already().dimmed()),
        StepResult::RankUp { .. } | StepResult::QuestCompleted { .. } => {}
    }
    if state.is_weakened() {
        grant_healing_potion(state, copy::lore_potion());
    }
}

fn run_challenge(state: &mut GameState, quest: Quest) -> io::Result<bool> {
    if state.step_done(quest.id, QuestStep::Challenge) {
        println!("{}", copy::challenge_already().dimmed());
        return Ok(false);
    }

    if state.is_weakened() {
        println!("{}", retro::dungeon_master_says(copy::too_weakened_to_fight()));
        return Ok(false);
    }

    let enc = narrative::encounter_for(quest);

    state.ownership_passed_first_try = quest.id == "ownership";
    state.errors_challenge_picked_unwrap = false;

    let mut questions: Vec<QuizQuestion> = quest.questions.to_vec();
    questions.push(quest.boss);

    let regular_count = quest.questions.len();
    let presented: Vec<PresentedQuestion> = questions
        .iter()
        .enumerate()
        .map(|(i, q)| q.present(quest.id, i as u32))
        .collect();

    println!();
    if let Some(e) = enc {
        println!("{}", retro::dungeon_master_says(e.challenge_open));
        println!(
            "{}",
            retro::enemy_says(e.enemy_emoji, e.enemy_name, e.enemy_taunt)
        );
        println!();
    } else {
        println!(
            "{}",
            retro::dungeon_master_says("A foe blocks the way — answer true to advance!")
        );
        println!();
    }

    let mut answers = Vec::new();
    let mut any_wrong = false;
    for (i, q) in presented.iter().enumerate() {
        let label = quest_combat_label(i, regular_count, enc);
        let Some(idx) = ask_quiz_question(q, &label)? else {
            println!("{}", copy::challenge_paused().dimmed());
            return Ok(false);
        };
        if q.is_bad_unwrap_pick(idx) {
            state.errors_challenge_picked_unwrap = true;
        }
        let correct = idx == q.correct;
        if !correct {
            any_wrong = true;
        }
        print_answer_result(q, idx, true);
        apply_quiz_heart(state, correct);
        answers.push(idx);
    }

    let passed = score_presented(&presented, &answers);

    if passed {
        let rank_before = state.rank();
        let today = today_string();
        state.complete_step(quest.id, QuestStep::Challenge, &today);
        let rank_after = state.rank();
        if let Some(e) = enc {
            println!("\n{}", retro::victory_flash(e.enemy_defeat));
            println!("{}", retro::success(&copy::quest_cleared(e.enemy_name)));
        }
        println!("{}", retro::success(copy::quiz_pass()));
        println!("{}", retro::tavern_cheer());
        println!(
            "{}",
            retro::success(&format!("+{XP_CHALLENGE} XP — foe vanquished!"))
        );
        // GAME: show even when rank-up replaced QuestCompleted in complete_step's return value.
        println!("{}", retro::success("The room falls silent — onward!"));
        if rank_after != rank_before {
            println!("{}", copy::rank_up(rank_after).green().bold());
        }
        if let Some(phase) = epic::newly_cleared_phase(state, quest.id) {
            try_dungeon_boss(state, phase)?;
        }
        if state.is_champion() && !state.victory_celebrated {
            epic::celebrate_champion(state);
            state.mark_victory_celebrated();
        }
        offer_quest_book(state, &quest, any_wrong)?;
        Ok(true)
    } else {
        if let Some(e) = enc {
            println!(
                "\n{}",
                retro::enemy_says(e.enemy_emoji, e.enemy_name, e.enemy_taunt)
            );
        }
        println!("{}", retro::failure(copy::quiz_fail()));
        state.ownership_passed_first_try = false;
        if any_wrong {
            offer_quest_book(state, &quest, true)?;
        }
        Ok(false)
    }
}

fn try_dungeon_boss(state: &mut GameState, phase: &'static epic::EpicPhase) -> io::Result<()> {
    if epic::boss_defeated(state, phase.id) {
        return Ok(());
    }
    println!();
    println!(
        "{}",
        format!(
            "⚔️  EPIC DOOR UNLOCKED: {} {} awaits in {}!",
            phase.boss_emoji, phase.boss_name, phase.name
        )
        .bright_yellow()
        .bold()
    );
    if !confirm_menu(
        &format!("🎲 Dungeon Master: Descend to face {}?", phase.boss_name),
        true,
    )?
    .unwrap_or(false)
    {
        println!(
            "{}",
            "The dungeon door groans shut — return when thou art ready."
                .dimmed()
                .italic()
        );
        return Ok(());
    }
    run_dungeon_boss(state, phase)
}

fn run_dungeon_boss(state: &mut GameState, phase: &'static epic::EpicPhase) -> io::Result<()> {
    if state.is_weakened() {
        println!("{}", retro::dungeon_master_says(copy::too_weakened_to_fight()));
        return Ok(());
    }
    epic::print_dungeon_intro(phase);
    let raw = epic::dungeon_boss_questions(phase);
    if raw.is_empty() {
        return Ok(());
    }

    let presented: Vec<PresentedQuestion> = raw
        .iter()
        .enumerate()
        .map(|(i, (quest_id, q))| q.present(quest_id, 100 + i as u32))
        .collect();

    let mut answers = Vec::new();
    for (i, q) in presented.iter().enumerate() {
        let is_final = i + 1 == presented.len();
        let label = if is_final {
            retro::final_gambit(phase.boss_emoji, phase.boss_name)
        } else {
            retro::boss_combat_round(i + 1, presented.len(), phase.boss_emoji)
        };
        let Some(idx) = ask_quiz_question(q, &label)? else {
            println!(
                "{}",
                "You retreat from the boss chamber — it waits in shadow."
                    .dimmed()
                    .italic()
            );
            return Ok(());
        };
        print_answer_result(q, idx, false);
        apply_quiz_heart(state, idx == q.correct);
        answers.push(idx);
    }

    if epic::score_dungeon_answers(&presented, &answers) {
        state.defeat_dungeon_boss(phase.id);
        state.xp += XP_DUNGEON_BOSS;
        println!(
            "\n{}",
            retro::success(&format!(
                "{} {} falls! +{XP_DUNGEON_BOSS} XP 🪙💎",
                phase.boss_emoji, phase.boss_name
            ))
        );
        println!("{}", retro::tavern_cheer());
    } else {
        println!(
            "{}",
            retro::failure(&format!(
                "{} shrugs off thy answers — study the phase scrolls!",
                phase.boss_name
            ))
        );
    }
    Ok(())
}
