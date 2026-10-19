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