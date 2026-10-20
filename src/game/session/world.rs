use super::{GameSession, LevelPhase, VICTORY_DURATION};
use crate::entities::{Boss, EnemyBullet, PowerUp, PowerUpType, Target, TargetType};
use macroquad::prelude::*;

const ZIGZAG_AMPLITUDE: f32 = 40.0;
const ZIGZAG_FREQUENCY: f32 = 3.0;

pub(super) fn update(session: &mut GameSession, delta: f32, viewport: Vec2) {
    if session.phase == LevelPhase::Wave {
        spawn_targets(session, viewport);
    }
    update_bullets(session, delta);
    update_targets(session, delta, viewport);
    update_power_ups(session, delta, viewport);
    update_boss(session, delta, viewport);
    resolve_collisions(session);
    remove_inactive_entities(session);
    if session.health > 0.0
        && session.phase == LevelPhase::Wave
        && session.level_kills >= session.kills_required()
    {
        start_boss(session, viewport);
    }
}

fn spawn_targets(session: &mut GameSession, viewport: Vec2) {
    if session.spawn_timer > session.spawn_interval {
        session.spawn_timer = 0.0;

        let target_type = match rand::gen_range(0, 3) {
            0 => TargetType::Normal,
            1 => TargetType::Fast,
            _ => TargetType::Zigzag,
        };

        let margin = if target_type.is_zigzag() {
            ZIGZAG_AMPLITUDE + 25.0
        } else {
            20.0 + 25.0
        };
        let random_x = if viewport.x > margin * 2.0 {
            rand::gen_range(margin, viewport.x - margin)
        } else {
            viewport.x / 2.0
        };

        session.targets.push(Target {
            rect: Rect::new(random_x - 25.0, -50.0, 50.0, 50.0),
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

fn update_targets(session: &mut GameSession, delta: f32, viewport: Vec2) {
    for target in &mut session.targets {
        target.elapsed += delta;
        target.rect.y += session.target_speed * target.target_type.speed_multiplier() * delta;
        if target.target_type.is_zigzag() {
            target.rect.x = target.origin_x
                + ZIGZAG_AMPLITUDE * (target.elapsed * ZIGZAG_FREQUENCY).sin()
                - target.rect.w / 2.0;
        }
        if target.rect.y > viewport.y {
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
                session.score += target.target_type.points() as u32;
                session.level_kills += 1;
                if session.level_kills.is_multiple_of(4) {
                    let kind = match (session.level_kills / 4 - 1) % 3 {
                        0 => PowerUpType::RapidFire,
                        1 => PowerUpType::MultiShot,
                        _ => PowerUpType::SpeedBoost,
                    };
                    let center = target.rect.center();
                    session.power_ups.push(PowerUp {
                        rect: Rect::new(center.x - 12.0, center.y - 12.0, 24.0, 24.0),
                        kind,
                    });
                }
                break;
            }
        }
    }

    if let Some(boss) = &mut session.boss {
        for bullet in session.bullets.iter_mut().filter(|bullet| bullet.active) {
            if boss.health > 0 && bullet.rect.overlaps(&boss.rect) {
                bullet.active = false;
                boss.health -= 1;
            }
        }
        if boss.health == 0 && session.health > 0.0 {
            session.score += 2000 * session.level;
            session.boss = None;
            session.enemy_bullets.clear();
            session.bullets.clear();
            session.power_ups.clear();
            session.phase = LevelPhase::Victory {
                remaining: VICTORY_DURATION,
            };
        }
    }
}

fn update_power_ups(session: &mut GameSession, delta: f32, viewport: Vec2) {
    session.power_ups.retain_mut(|power_up| {
        power_up.rect.y += 100.0 * delta;
        power_up.rect.x = power_up
            .rect
            .x
            .clamp(0.0, (viewport.x - power_up.rect.w).max(0.0));
        if power_up.rect.overlaps(&session.player_rect) {
            session.effects.apply(power_up.kind);
            false
        } else {
            power_up.rect.y <= viewport.y
        }
    });
}

fn boss_bounds(viewport: Vec2) -> Rect {
    let width = 140.0_f32.min(viewport.x);
    let player_y = (viewport.y - 160.0).max(0.0);
    let y = (player_y - 75.0).clamp(120.0, 135.0);
    let height = (player_y - y - 10.0).clamp(12.0, 55.0);
    Rect::new((viewport.x - width) / 2.0, y, width, height)
}

fn start_boss(session: &mut GameSession, viewport: Vec2) {
    session.phase = LevelPhase::Boss;
    session.spawn_timer = 0.0;
    session.targets.clear();
    session.bullets.clear();
    let max_health = 30 + (session.level - 1) * 10;
    session.boss = Some(Boss {
        rect: boss_bounds(viewport),
        health: max_health,
        max_health,
        direction: 1.0,
        attack_timer: 0.0,
    });
}

fn update_boss(session: &mut GameSession, delta: f32, viewport: Vec2) {
    if let Some(boss) = &mut session.boss {
        let bounds = boss_bounds(viewport);
        boss.rect.w = bounds.w;
        boss.rect.h = bounds.h;
        boss.rect.y = bounds.y;
        let speed = (100.0 + (session.level - 1) as f32 * 15.0).min(260.0);
        let max_x = (viewport.x - boss.rect.w).max(0.0);
        boss.rect.x = (boss.rect.x + boss.direction * speed * delta).clamp(0.0, max_x);
        if boss.rect.x >= max_x {
            boss.direction = -1.0;
        } else if boss.rect.x <= 0.0 {
            boss.direction = 1.0;
        }
        boss.attack_timer += delta;
        let interval = (1.2 - (session.level - 1) as f32 * 0.08).max(0.4);
        if boss.attack_timer >= interval {
            boss.attack_timer %= interval;
            let speed = (210.0 + (session.level - 1) as f32 * 15.0).min(400.0);
            for horizontal in [-0.55, 0.0, 0.55] {
                session.enemy_bullets.push(EnemyBullet {
                    rect: Rect::new(boss.rect.center().x - 5.0, boss.rect.bottom(), 10.0, 16.0),
                    velocity: vec2(horizontal * speed, speed),
                });
            }
        }
    }
    session.enemy_bullets.retain_mut(|bullet| {
        bullet.rect.x += bullet.velocity.x * delta;
        bullet.rect.y += bullet.velocity.y * delta;
        if bullet.rect.overlaps(&session.player_rect) {
            session.health = (session.health - 10.0).max(0.0);
            false
        } else {
            bullet.rect.y <= viewport.y && bullet.rect.right() >= 0.0 && bullet.rect.x <= viewport.x
        }
    });
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

    for power_up in &session.power_ups {
        let rect = power_up.rect;
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, power_up.kind.color());
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, WHITE);
        draw_text(
            power_up.kind.label(),
            rect.x + 5.0,
            rect.y + 19.0,
            24.0,
            BLACK,
        );
    }

    if let Some(boss) = &session.boss {
        let rect = boss.rect;
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, PURPLE);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, PINK);
        draw_circle(rect.center().x, rect.center().y, 14.0, RED);
        for x in [rect.x + 12.0, rect.right() - 12.0] {
            draw_rectangle(x - 5.0, rect.bottom() - 5.0, 10.0, 15.0, ORANGE);
        }
    }

    for bullet in &session.enemy_bullets {
        let rect = bullet.rect;
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, PINK);
    }
}
