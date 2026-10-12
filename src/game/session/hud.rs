use super::{GameSession, MAX_HEALTH};
use macroquad::prelude::*;

pub(super) fn draw(session: &GameSession) {
    draw_text(format!("SCORE: {}", session.score), 20.0, 40.0, 30.0, WHITE);

    let bar_width = 150.0;
    let bar_height = 15.0;
    let bar_x = screen_width() - bar_width - 20.0;
    let bar_y = 25.0;

    draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);
    let current_health_width = (session.health / MAX_HEALTH) * bar_width;
    let health_color = if session.health > 50.0 {
        GREEN
    } else if session.health > 20.0 {
        YELLOW
    } else {
        RED
    };
    draw_rectangle(bar_x, bar_y, current_health_width, bar_height, health_color);
}
