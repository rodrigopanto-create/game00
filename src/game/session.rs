mod hud;
mod player;
mod world;

use crate::entities::{Boss, Bullet, EnemyBullet, PowerUp, PowerUpEffects, Target};
use macroquad::prelude::*;

const MAX_HEALTH: f32 = 100.0;
const INITIAL_TARGET_SPEED: f32 = 150.0;
const VICTORY_DURATION: f32 = 2.5;

#[derive(Debug, PartialEq)]
enum LevelPhase {
    Wave,
    Boss,
    Victory { remaining: f32 },
}

pub(super) struct GameSession {
    pub(super) player_rect: Rect,
    pub(super) bullets: Vec<Bullet>,
    pub(super) targets: Vec<Target>,
    pub(super) health: f32,
    pub(super) score: u32,
    pub(super) spawn_timer: f32,
    game_time: f32,
    target_speed: f32,
    spawn_interval: f32,
    power_ups: Vec<PowerUp>,
    effects: PowerUpEffects,
    shot_cooldown: f32,
    boss: Option<Boss>,
    enemy_bullets: Vec<EnemyBullet>,
    level: u32,
    level_kills: u32,
    phase: LevelPhase,
}

impl GameSession {
    pub(super) fn new() -> Self {
        Self::new_for_viewport(vec2(screen_width(), screen_height()))
    }

    fn new_for_viewport(viewport: Vec2) -> Self {
        let mut session = Self {
            player_rect: Rect::new(0.0, 0.0, 40.0, 40.0),
            bullets: Vec::new(),
            targets: Vec::new(),
            health: MAX_HEALTH,
            score: 0,
            spawn_timer: 0.0,
            game_time: 0.0,
            target_speed: INITIAL_TARGET_SPEED,
            spawn_interval: 1.0,
            power_ups: Vec::new(),
            effects: PowerUpEffects::default(),
            shot_cooldown: 0.0,
            boss: None,
            enemy_bullets: Vec::new(),
            level: 1,
            level_kills: 0,
            phase: LevelPhase::Wave,
        };
        session.reset_for_viewport(viewport);
        session
    }

    pub(super) fn reset(&mut self) {
        self.reset_for_viewport(vec2(screen_width(), screen_height()));
    }

    fn reset_for_viewport(&mut self, viewport: Vec2) {
        self.health = MAX_HEALTH;
        self.score = 0;
        self.game_time = 0.0;
        self.spawn_timer = 0.0;
        self.target_speed = INITIAL_TARGET_SPEED;
        self.spawn_interval = 1.0;
        self.bullets.clear();
        self.targets.clear();
        self.power_ups.clear();
        self.effects = PowerUpEffects::default();
        self.shot_cooldown = 0.0;
        self.boss = None;
        self.enemy_bullets.clear();
        self.level = 1;
        self.level_kills = 0;
        self.phase = LevelPhase::Wave;
        self.player_rect = Rect::new(
            (viewport.x / 2.0 - 20.0).max(0.0),
            (viewport.y - 160.0).max(0.0),
            40.0,
            40.0,
        );
    }

    pub(super) fn score(&self) -> u32 {
        self.score
    }

    /// Actualiza la partida y devuelve `true` cuando el jugador pierde toda la vida.
    pub(super) fn update(&mut self, delta: f32, controls: &super::Controls) -> bool {
        self.update_for_viewport(delta, controls, vec2(screen_width(), screen_height()))
    }

    fn kills_required(&self) -> u32 {
        12 + (self.level - 1) * 4
    }

    fn update_for_viewport(
        &mut self,
        delta: f32,
        controls: &super::Controls,
        viewport: Vec2,
    ) -> bool {
        if let LevelPhase::Victory { remaining } = &mut self.phase {
            *remaining -= delta;
            if *remaining <= 0.0 {
                self.level += 1;
                self.level_kills = 0;
                self.game_time = 0.0;
                self.spawn_timer = 0.0;
                self.phase = LevelPhase::Wave;
            }
            return false;
        }
        self.spawn_timer += delta;
        self.game_time += delta;
        self.effects.tick(delta);
        self.shot_cooldown = (self.shot_cooldown - delta).max(0.0);

        self.target_speed =
            (INITIAL_TARGET_SPEED + (self.level - 1) as f32 * 30.0 + self.game_time * 6.0)
                .min(550.0);
        self.spawn_interval =
            (1.0 - (self.level - 1) as f32 * 0.1 - self.game_time * 0.012).max(0.3);

        player::update(self, delta, controls, viewport);
        world::update(self, delta, viewport);
        self.health <= 0.0
    }

    pub(super) fn draw(&self) {
        clear_background(BLACK);
        player::draw(self);
        world::draw(self);
        hud::draw(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{PowerUpType, TargetType};
    use crate::game::Controls;

    const VIEWPORT: Vec2 = Vec2::new(800.0, 600.0);

    fn target(rect: Rect) -> Target {
        Target {
            rect,
            active: true,
            target_type: TargetType::Normal,
            origin_x: rect.center().x,
            elapsed: 0.0,
        }
    }

    fn bullet(rect: Rect) -> Bullet {
        Bullet {
            rect,
            active: true,
            speed: 600.0,
        }
    }

    #[test]
    fn effects_refresh_coexist_and_expire_to_base_stats() {
        let mut effects = PowerUpEffects::default();
        assert_eq!(effects.shot_interval(), 0.25);
        assert_eq!(effects.shot_count(), 1);
        assert_eq!(effects.movement_speed(), 400.0);
        effects.apply(PowerUpType::RapidFire);
        effects.apply(PowerUpType::MultiShot);
        effects.apply(PowerUpType::SpeedBoost);
        assert_eq!(effects.shot_interval(), 0.1);
        assert_eq!(effects.shot_count(), 3);
        assert_eq!(effects.movement_speed(), 600.0);
        effects.tick(3.0);
        effects.apply(PowerUpType::RapidFire);
        assert_eq!(effects.rapid_fire, 8.0);
        assert_eq!(effects.multi_shot, 5.0);
        effects.tick(5.0);
        assert_eq!(effects.shot_count(), 1);
        assert_eq!(effects.movement_speed(), 400.0);
        effects.tick(3.0);
        assert_eq!(effects.shot_interval(), 0.25);
    }

    #[test]
    fn held_fire_obeys_cooldown_and_rapid_fire_increases_actual_shots() {
        let controls = Controls {
            action_down: true,
            ..Controls::default()
        };
        let viewport = vec2(800.0, 1600.0);
        let mut normal = GameSession::new_for_viewport(viewport);
        let mut rapid = GameSession::new_for_viewport(viewport);
        rapid.effects.apply(PowerUpType::RapidFire);
        for _ in 0..10 {
            normal.update_for_viewport(0.11, &controls, viewport);
            rapid.update_for_viewport(0.11, &controls, viewport);
        }
        assert_eq!(normal.bullets.len(), 4);
        assert_eq!(rapid.bullets.len(), 10);
        let count = rapid.bullets.len();
        rapid.update_for_viewport(0.0, &controls, viewport);
        assert_eq!(rapid.bullets.len(), count);
    }

    #[test]
    fn multi_shot_fires_three_distinct_bullets_and_speed_boost_moves_faster() {
        let mut session = GameSession::new_for_viewport(VIEWPORT);
        session.effects.apply(PowerUpType::MultiShot);
        session.effects.apply(PowerUpType::SpeedBoost);
        let start = session.player_rect.x;
        let controls = Controls {
            right: true,
            action_pressed: true,
            ..Controls::default()
        };
        session.update_for_viewport(0.1, &controls, VIEWPORT);
        assert_eq!(session.bullets.len(), 3);
        assert!(session
            .bullets
            .windows(2)
            .all(|pair| pair[0].rect.x < pair[1].rect.x));
        assert_eq!(session.player_rect.x - start, 60.0);
    }

    #[test]
    fn every_fourth_kill_drops_alternating_collectibles_at_enemy_position() {
        for (kills, kind) in [
            (3, PowerUpType::RapidFire),
            (7, PowerUpType::MultiShot),
            (11, PowerUpType::SpeedBoost),
        ] {
            let mut session = GameSession::new_for_viewport(VIEWPORT);
            session.level_kills = kills;
            let rect = Rect::new(200.0, 200.0, 50.0, 50.0);
            session.targets.push(target(rect));
            session.bullets.push(bullet(rect));
            world::update(&mut session, 0.0, VIEWPORT);
            assert_eq!(session.power_ups.len(), 1);
            assert_eq!(session.power_ups[0].kind, kind);
            assert_eq!(session.power_ups[0].rect.center(), rect.center());
            assert_eq!(session.score, 100);
        }
    }

    #[test]
    fn a_bullet_only_destroys_one_overlapping_enemy() {
        let mut session = GameSession::new_for_viewport(VIEWPORT);
        let rect = Rect::new(200.0, 200.0, 50.0, 50.0);
        session.targets.extend([target(rect), target(rect)]);
        session.bullets.push(bullet(rect));
        world::update(&mut session, 0.0, VIEWPORT);
        assert_eq!(session.targets.len(), 1);
        assert_eq!(session.level_kills, 1);
        assert_eq!(session.score, 100);
    }

    #[test]
    fn collectibles_fall_apply_only_on_contact_and_leave_screen() {
        let mut session = GameSession::new_for_viewport(VIEWPORT);
        session.power_ups.push(PowerUp {
            rect: Rect::new(
                session.player_rect.x,
                session.player_rect.y - 30.0,
                24.0,
                24.0,
            ),
            kind: PowerUpType::SpeedBoost,
        });
        world::update(&mut session, 0.05, VIEWPORT);
        assert_eq!(session.power_ups.len(), 1);
        assert_eq!(session.effects.speed_boost, 0.0);
        world::update(&mut session, 0.05, VIEWPORT);
        assert!(session.power_ups.is_empty());
        assert_eq!(session.effects.speed_boost, PowerUpEffects::DURATION);
        session.power_ups.push(PowerUp {
            rect: Rect::new(0.0, VIEWPORT.y, 24.0, 24.0),
            kind: PowerUpType::RapidFire,
        });
        world::update(&mut session, 0.1, VIEWPORT);
        assert!(session.power_ups.is_empty());
        assert_eq!(session.effects.rapid_fire, 0.0);
    }

    #[test]
    fn escaped_enemies_damage_player_but_do_not_count_as_kills() {
        let mut session = GameSession::new_for_viewport(VIEWPORT);
        session
            .targets
            .push(target(Rect::new(0.0, VIEWPORT.y + 1.0, 50.0, 50.0)));
        world::update(&mut session, 0.0, VIEWPORT);
        assert_eq!(session.health, MAX_HEALTH - 15.0);
        assert_eq!(session.level_kills, 0);
        assert_eq!(session.score, 0);
        assert!(session.power_ups.is_empty());
    }

    fn boss_session(viewport: Vec2) -> GameSession {
        let mut session = GameSession::new_for_viewport(viewport);
        session.level_kills = session.kills_required();
        world::update(&mut session, 0.0, viewport);
        session
    }

    #[test]
    fn boss_starts_at_exact_threshold_and_stops_normal_spawning() {
        let mut session = GameSession::new_for_viewport(VIEWPORT);
        session.level_kills = session.kills_required() - 1;
        world::update(&mut session, 0.0, VIEWPORT);
        assert!(session.boss.is_none());
        let rect = Rect::new(200.0, 200.0, 50.0, 50.0);
        session.targets.extend([target(rect), target(rect)]);
        session.bullets.push(bullet(rect));
        world::update(&mut session, 0.0, VIEWPORT);
        assert_eq!(session.phase, LevelPhase::Boss);
        assert!(session.targets.is_empty());
        assert_eq!(session.boss.as_ref().unwrap().health, 30);
        session.spawn_timer = 100.0;
        session.boss.as_mut().unwrap().attack_timer = 1.2;
        world::update(&mut session, 0.0, VIEWPORT);
        assert!(session.targets.is_empty());
        assert_eq!(session.enemy_bullets.len(), 3);
        assert!(session.enemy_bullets[0].velocity.x < 0.0);
        assert_eq!(session.enemy_bullets[1].velocity.x, 0.0);
        assert!(session.enemy_bullets[2].velocity.x > 0.0);
    }

    #[test]
    fn boss_reverses_at_edges_and_stays_visible_after_resize() {
        let mut session = boss_session(VIEWPORT);
        let boss = session.boss.as_mut().unwrap();
        boss.rect.x = VIEWPORT.x - boss.rect.w;
        world::update(&mut session, 0.1, VIEWPORT);
        assert_eq!(session.boss.as_ref().unwrap().direction, -1.0);
        world::update(&mut session, 0.1, vec2(320.0, 600.0));
        let boss = session.boss.as_ref().unwrap();
        assert!(boss.rect.x >= 0.0 && boss.rect.right() <= 320.0);
    }

    #[test]
    fn boss_fits_between_hud_and_player_in_mobile_portrait_and_landscape() {
        for viewport in [vec2(320.0, 640.0), vec2(568.0, 320.0)] {
            let session = boss_session(viewport);
            let rect = session.boss.as_ref().unwrap().rect;
            assert!(rect.y > 118.0);
            assert!(rect.bottom() + 10.0 <= session.player_rect.y);
            assert!(rect.x >= 0.0 && rect.right() <= viewport.x);
        }
    }

    #[test]
    fn boss_damage_victory_and_next_level_are_applied_once() {
        let mut session = boss_session(VIEWPORT);
        let rect = session.boss.as_ref().unwrap().rect;
        session.bullets.push(bullet(rect));
        world::update(&mut session, 0.0, VIEWPORT);
        assert_eq!(session.boss.as_ref().unwrap().health, 29);
        for _ in 0..35 {
            session.bullets.push(bullet(rect));
        }
        world::update(&mut session, 0.0, VIEWPORT);
        assert!(session.boss.is_none());
        assert!(session.bullets.is_empty());
        assert!(session.enemy_bullets.is_empty());
        assert_eq!(session.score, 2000);
        assert_eq!(
            session.phase,
            LevelPhase::Victory {
                remaining: VICTORY_DURATION
            }
        );
        let controls = Controls {
            action_down: true,
            ..Controls::default()
        };
        session.update_for_viewport(VICTORY_DURATION / 2.0, &controls, VIEWPORT);
        assert!(session.bullets.is_empty());
        assert_eq!(session.level, 1);
        session.update_for_viewport(VICTORY_DURATION / 2.0, &controls, VIEWPORT);
        assert_eq!(session.level, 2);
        assert_eq!(session.phase, LevelPhase::Wave);
        assert_eq!(session.level_kills, 0);
        assert_eq!(session.score, 2000);
        session.update_for_viewport(0.0, &Controls::default(), VIEWPORT);
        assert!(session.target_speed > INITIAL_TARGET_SPEED);
        assert!(session.spawn_interval < 1.0);
        assert_eq!(session.kills_required(), 16);
        session.level_kills = session.kills_required();
        world::update(&mut session, 0.0, VIEWPORT);
        assert_eq!(session.boss.as_ref().unwrap().health, 40);
    }

    #[test]
    fn hostile_bullets_damage_once_and_can_end_the_game() {
        let mut session = boss_session(VIEWPORT);
        session.health = 10.0;
        session.enemy_bullets.push(EnemyBullet {
            rect: session.player_rect,
            velocity: Vec2::ZERO,
        });
        assert!(session.update_for_viewport(0.0, &Controls::default(), VIEWPORT));
        assert_eq!(session.health, 0.0);
        assert!(session.enemy_bullets.is_empty());
    }

    #[test]
    fn reset_clears_all_new_state_and_mobile_ship_avoids_controls() {
        let viewport = vec2(320.0, 640.0);
        let mut session = boss_session(viewport);
        session.effects.apply(PowerUpType::MultiShot);
        session.power_ups.push(PowerUp {
            rect: session.player_rect,
            kind: PowerUpType::RapidFire,
        });
        session.enemy_bullets.push(EnemyBullet {
            rect: session.player_rect,
            velocity: Vec2::ZERO,
        });
        session.level = 3;
        session.reset_for_viewport(viewport);
        assert_eq!(session.level, 1);
        assert_eq!(session.phase, LevelPhase::Wave);
        assert_eq!(session.effects.shot_count(), 1);
        assert!(session.boss.is_none());
        assert!(session.power_ups.is_empty());
        assert!(session.enemy_bullets.is_empty());
        let controls = Controls {
            right: true,
            action_down: true,
            ..Controls::default()
        };
        session.update_for_viewport(1.0, &controls, viewport);
        assert!(session.player_rect.right() <= viewport.x);
        assert!(session.player_rect.bottom() <= viewport.y - 120.0);
    }
}
