use super::GameSession;
use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 400.0;
const SHIP_COLOR: Color = Color::new(0.15, 0.85, 0.35, 1.0);

pub(super) fn update(session: &mut GameSession, delta: f32, controls: &crate::game::Controls) {
    if controls.right {
        session.player_rect.x += PLAYER_SPEED * delta;
    }
    if controls.left {
        session.player_rect.x -= PLAYER_SPEED * delta;
    }

    session.player_rect.x = session
        .player_rect
        .x
        .clamp(0.0, screen_width() - session.player_rect.w);

    if controls.action_pressed {
        session.bullets.push(crate::entities::Bullet {
            rect: Rect::new(
                session.player_rect.x + session.player_rect.w / 2.0 - 4.0,
                session.player_rect.y,
                8.0,
                16.0,
            ),
            speed: 600.0,
            active: true,
        });
    }
}

pub(super) fn draw(session: &GameSession) {
    let rect = session.player_rect;
    draw_triangle(
        vec2(rect.x + rect.w / 2.0, rect.y),
        vec2(rect.x, rect.y + rect.h),
        vec2(rect.x + rect.w, rect.y + rect.h),
        SHIP_COLOR,
    );
}
