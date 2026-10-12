mod hud;
mod player;
mod world;

use crate::entities::{Bullet, Target};
use macroquad::prelude::*;

const MAX_HEALTH: f32 = 100.0;
const INITIAL_TARGET_SPEED: f32 = 150.0;

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
}

impl GameSession {
    pub(super) fn new() -> Self {
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
        };
        session.reset(); // Asigna las posiciones correctas según el tamaño de pantalla actual
        session
    }

    pub(super) fn reset(&mut self) {
        self.health = MAX_HEALTH;
        self.score = 0;
        self.game_time = 0.0;
        self.spawn_timer = 0.0;
        self.target_speed = INITIAL_TARGET_SPEED;
        self.spawn_interval = 1.0;
        self.bullets.clear();
        self.targets.clear();
        self.player_rect = Rect::new(
            screen_width() / 2.0 - 20.0,
            screen_height() - 60.0,
            40.0,
            40.0,
        );
    }

    pub(super) fn score(&self) -> u32 {
        self.score
    }

    /// Actualiza la partida y devuelve `true` cuando el jugador pierde toda la vida.
    pub(super) fn update(&mut self, delta: f32) -> bool {
        self.spawn_timer += delta;
        self.game_time += delta;

        self.target_speed = (INITIAL_TARGET_SPEED + self.game_time * 6.0).min(550.0);
        self.spawn_interval = (1.0 - self.game_time * 0.012).max(0.3);

        player::update(self, delta);
        world::update(self, delta);
        self.health <= 0.0
    }

    pub(super) fn draw(&self) {
        clear_background(BLACK);
        player::draw(self);
        world::draw(self);
        hud::draw(self);
    }
}
