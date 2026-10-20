use super::{GameSession, LevelPhase, MAX_HEALTH};
use crate::entities::PowerUpType;
use macroquad::prelude::*;

pub(super) fn draw(session: &GameSession) {
    draw_text(format!("SCORE: {}", session.score), 16.0, 28.0, 24.0, WHITE);

    let bar_width = (screen_width() * 0.3).min(150.0);
    let bar_height = 12.0;
    let bar_x = screen_width() - bar_width - 16.0;
    let bar_y = 38.0;

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

    let level_text = if session.phase == LevelPhase::Wave {
        format!(
            "LV {}  {}/{}",
            session.level,
            session.level_kills,
            session.kills_required()
        )
    } else {
        format!("LV {}", session.level)
    };
    draw_text(&level_text, 16.0, 50.0, 20.0, WHITE);

    let mut x = 16.0;
    for (kind, timer) in [
        (PowerUpType::RapidFire, session.effects.rapid_fire),
        (PowerUpType::MultiShot, session.effects.multi_shot),
        (PowerUpType::SpeedBoost, session.effects.speed_boost),
    ] {
        if timer > 0.0 {
            draw_text(
                format!("{}:{:.0}s", kind.label(), timer.ceil()),
                x,
                74.0,
                20.0,
                kind.color(),
            );
            x += 82.0;
        }
    }

    if let Some(boss) = &session.boss {
        let width = (screen_width() - 32.0).max(0.0);
        draw_text("BOSS", 16.0, 98.0, 22.0, PINK);
        draw_rectangle(16.0, 106.0, width, 12.0, DARKGRAY);
        draw_rectangle(
            16.0,
            106.0,
            width * boss.health as f32 / boss.max_health as f32,
            12.0,
            RED,
        );
    }

    if matches!(session.phase, LevelPhase::Victory { .. }) {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.75),
        );
        for (text, y, color) in [
            (
                format!("NIVEL {} SUPERADO", session.level),
                screen_height() * 0.4,
                GOLD,
            ),
            (
                format!("SIGUIENTE: NIVEL {}", session.level + 1),
                screen_height() * 0.4 + 40.0,
                WHITE,
            ),
        ] {
            let font_size = (screen_width() / 14.0).clamp(12.0, 32.0);
            let size = measure_text(&text, None, font_size as u16, 1.0);
            draw_text(
                &text,
                (screen_width() - size.width) / 2.0,
                y,
                font_size,
                color,
            );
        }
    }
}
