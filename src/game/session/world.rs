use super::GameSession;
use crate::entities::{Target, TargetType};
use macroquad::prelude::*;

const ZIGZAG_AMPLITUDE: f32 = 40.0;
const ZIGZAG_FREQUENCY: f32 = 3.0;

pub(super) fn update(session: &mut GameSession, delta: f32) {
    spawn_targets(session);
    update_bullets(session, delta);
    update_targets(session, delta);
    resolve_collisions(session);
    remove_inactive_entities(session);
}

fn spawn_targets(session: &mut GameSession) {
    if session.spawn_timer > session.spawn_interval {
        session.spawn_timer = 0.0;
        let target_type = match rand::gen_range(0, 3) {
            0 => TargetType::Standard,
            1 => TargetType::Fast,
            _ => TargetType::ZigZag,
        };
        let margin = if target_type.is_zigzag() {
            ZIGZAG_AMPLITUDE + target_type.size() / 2.0
        } else {
            20.0 + target_type.size() / 2.0
        };
        let random_x = rand::gen_range(margin, screen_width() - margin);
        session.targets.push(Target {
            rect: Rect::new(
                random_x - target_type.size() / 2.0,
                -target_type.size(),
                target_type.size(),
                target_type.size(),
            ),
            active: true,
            target_type,
            origin_x: random_x,
            elapsed: 0.0,
        });
    }
}

fn update_bullets(session: &mut GameSession, delta: f32) {
    for bullet in &mut session.bullets {
        bullet.rect.y -= bullet.speed * delta;
        if bullet.rect.y < -20.0 {
            bullet.active = false;
        }
    }
}

fn update_targets(session: &mut GameSession, delta: f32) {
    for target in &mut session.targets {
        target.elapsed += delta;
        target.rect.y += session.target_speed * target.target_type.speed_multiplier() * delta;
        if target.target_type.is_zigzag() {
            target.rect.x = target.origin_x
                + ZIGZAG_AMPLITUDE * (target.elapsed * ZIGZAG_FREQUENCY).sin()
                - target.rect.w / 2.0;
        }
        if target.rect.y > screen_height() {
            target.active = false;
            session.health -= 15.0;
            if session.health <= 0.0 {
                session.health = 0.0;
            }
        }
    }
}

fn resolve_collisions(session: &mut GameSession) {
    for bullet in session.bullets.iter_mut().filter(|bullet| bullet.active) {
        for target in session.targets.iter_mut().filter(|target| target.active) {
            if bullet.rect.overlaps(&target.rect) {
                bullet.active = false;
                target.active = false;
                session.score += target.target_type.points();
            }
        }
    }
}

fn remove_inactive_entities(session: &mut GameSession) {
    session.bullets.retain(|bullet| bullet.active);
    session.targets.retain(|target| target.active);
}

pub(super) fn draw(session: &GameSession) {
    for bullet in &session.bullets {
        draw_rectangle(
            bullet.rect.x,
            bullet.rect.y,
            bullet.rect.w,
            bullet.rect.h,
            YELLOW,
        );
    }

    for target in &session.targets {
        draw_rectangle(
            target.rect.x,
            target.rect.y,
            target.rect.w,
            target.rect.h,
            target.target_type.color(),
        );
    }
}
