use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TargetType {
    Normal,
    Fast,
    Zigzag,
}

impl TargetType {
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            TargetType::Normal => 1.0,
            TargetType::Fast => 1.5,
            TargetType::Zigzag => 1.2,
        }
    }

    pub fn points(&self) -> i32 {
        match self {
            TargetType::Normal => 100,
            TargetType::Fast => 200,
            TargetType::Zigzag => 300,
        }
    }

    pub fn is_zigzag(&self) -> bool {
        matches!(self, TargetType::Zigzag)
    }

    pub fn color(&self) -> Color {
        match self {
            TargetType::Normal => YELLOW,
            TargetType::Fast => RED,
            TargetType::Zigzag => GREEN,
        }
    }
}

pub struct Target {
    pub rect: Rect,
    pub active: bool,
    pub target_type: TargetType,
    pub origin_x: f32,
    pub elapsed: f32,
}
pub struct Bullet {
    pub rect: Rect,
    pub active: bool,
    pub speed: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUpType {
    RapidFire,
    MultiShot,
    SpeedBoost,
}

impl PowerUpType {
    pub fn label(self) -> &'static str {
        match self {
            Self::RapidFire => "R",
            Self::MultiShot => "M",
            Self::SpeedBoost => "S",
        }
    }

    pub fn color(self) -> Color {
        match self {
            Self::RapidFire => ORANGE,
            Self::MultiShot => SKYBLUE,
            Self::SpeedBoost => LIME,
        }
    }
}

pub struct PowerUp {
    pub rect: Rect,
    pub kind: PowerUpType,
}

#[derive(Default)]
pub struct PowerUpEffects {
    pub rapid_fire: f32,
    pub multi_shot: f32,
    pub speed_boost: f32,
}

impl PowerUpEffects {
    pub const DURATION: f32 = 8.0;

    pub fn apply(&mut self, kind: PowerUpType) {
        *match kind {
            PowerUpType::RapidFire => &mut self.rapid_fire,
            PowerUpType::MultiShot => &mut self.multi_shot,
            PowerUpType::SpeedBoost => &mut self.speed_boost,
        } = Self::DURATION;
    }

    pub fn tick(&mut self, delta: f32) {
        for timer in [
            &mut self.rapid_fire,
            &mut self.multi_shot,
            &mut self.speed_boost,
        ] {
            *timer = (*timer - delta).max(0.0);
        }
    }

    pub fn shot_interval(&self) -> f32 {
        if self.rapid_fire > 0.0 {
            0.1
        } else {
            0.25
        }
    }

    pub fn shot_count(&self) -> usize {
        if self.multi_shot > 0.0 {
            3
        } else {
            1
        }
    }

    pub fn movement_speed(&self) -> f32 {
        if self.speed_boost > 0.0 {
            600.0
        } else {
            400.0
        }
    }
}

pub struct Boss {
    pub rect: Rect,
    pub health: u32,
    pub max_health: u32,
    pub direction: f32,
    pub attack_timer: f32,
}

pub struct EnemyBullet {
    pub rect: Rect,
    pub velocity: Vec2,
}
