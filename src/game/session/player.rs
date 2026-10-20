use super::GameSession;
use macroquad::prelude::*;

const SHIP_COLOR: Color = Color::new(0.15, 0.85, 0.35, 1.0);

pub(super) fn update(
    session: &mut GameSession,
    delta: f32,
    controls: &crate::game::Controls,
    viewport: Vec2,
) {
    let speed = session.effects.movement_speed();
    if controls.right {
        session.player_rect.x += speed * delta;
    }
    if controls.left {
        session.player_rect.x -= speed * delta;
    }

    session.player_rect.x = session
        .player_rect
        .x
        .clamp(0.0, (viewport.x - session.player_rect.w).max(0.0));
    session.player_rect.y = (viewport.y - 160.0).max(0.0);

    if (controls.action_pressed || controls.action_down) && session.shot_cooldown <= 0.0 {
        let count = session.effects.shot_count();
        for index in 0..count {
            let offset = (index as f32 - (count - 1) as f32 / 2.0) * 14.0;
            session.bullets.push(crate::entities::Bullet {
                rect: Rect::new(
                    (session.player_rect.center().x - 4.0 + offset)
                        .clamp(0.0, (viewport.x - 8.0).max(0.0)),
                    session.player_rect.y,
                    8.0,
                    16.0,
                ),
                speed: 600.0,
                active: true,
            });
        }
        session.shot_cooldown = session.effects.shot_interval();
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
