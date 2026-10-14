mod controls;
mod highscores;
mod screens;
mod session;

use controls::Controls;
use highscores::{HighScores, InitialsEntry};
use macroquad::prelude::*;
use screens::{draw_game_over, draw_high_scores, draw_new_record, draw_start_screen};
use session::GameSession;

/// Tiempo en que se ignora Fire tras cambiar de pantalla, para evitar saltarla sin querer.
const INPUT_GRACE: f32 = 0.5;
const HIGH_SCORES_DURATION: f32 = 10.0;

enum GameState {
    StartScreen,
    Playing,
    GameOver,
    NewRecord(InitialsEntry),
    HighScores { highlight: Option<usize> },
}

/// Ejecuta el loop principal del juego.
pub async fn run() {
    let mut game_state = GameState::StartScreen;
    let mut session = GameSession::new();
    let mut high_scores = HighScores::load();
    let mut blink_timer = 0.0;
    let mut state_time = 0.0;

    loop {
        let delta = get_frame_time();
        let controls = Controls::poll();
        state_time += delta;
        blink_timer += delta;
        let accepts_input = state_time >= INPUT_GRACE;
        let mut next_state = None;

        match &mut game_state {
            GameState::StartScreen => {
                if controls.action_pressed {
                    session.reset();
                    next_state = Some(GameState::Playing);
                }

                draw_start_screen(blink_timer);
            }
            GameState::Playing => {
                if session.update(delta, &controls) {
                    next_state = Some(if high_scores.qualifies(session.score()) {
                        GameState::NewRecord(InitialsEntry::new())
                    } else {
                        GameState::GameOver
                    });
                }

                session.draw();
            }
            GameState::GameOver => {
                draw_game_over(session.score(), blink_timer);

                if accepts_input && controls.action_pressed {
                    next_state = Some(GameState::HighScores { highlight: None });
                }
            }
            GameState::NewRecord(entry) => {
                if accepts_input {
                    if let Some(initials) = entry.update(&controls, delta) {
                        let highlight = high_scores.insert(initials, session.score());
                        next_state = Some(GameState::HighScores { highlight });
                    }
                }

                draw_new_record(entry, session.score(), blink_timer);
            }
            GameState::HighScores { highlight } => {
                draw_high_scores(&high_scores, *highlight, blink_timer);

                if (accepts_input && controls.action_pressed) || state_time >= HIGH_SCORES_DURATION {
                    session.reset();
                    next_state = Some(GameState::StartScreen);
                }
            }
        }

        if let Some(state) = next_state {
            game_state = state;
            state_time = 0.0;
            blink_timer = 0.0;
        }

        controls.draw();

        next_frame().await;
    }
}
