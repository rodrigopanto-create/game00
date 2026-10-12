mod screens;
mod session;

use macroquad::prelude::*;
use screens::{draw_game_over, draw_start_screen};
use session::GameSession;

#[derive(PartialEq)]
enum GameState {
    StartScreen,
    Playing,
    GameOver,
}

/// Ejecuta el loop principal del juego.
pub async fn run() {
    let mut game_state = GameState::StartScreen;
    let mut session = GameSession::new();
    let mut blink_timer = 0.0;

    loop {
        let delta = get_frame_time();

        match game_state {
            GameState::StartScreen => {
                blink_timer += delta;

                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    session.reset();
                    game_state = GameState::Playing;
                }

                draw_start_screen(blink_timer);
            }
            GameState::Playing => {
                if session.update(delta) {
                    game_state = GameState::GameOver;
                }

                session.draw();
            }
            GameState::GameOver => {
                blink_timer += delta;
                draw_game_over(session.score(), blink_timer);

                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    session.reset();
                    game_state = GameState::StartScreen;
                }
            }
        }

        next_frame().await;
    }
}
