use macroquad::prelude::*;

/// Bala disparada por el jugador.
pub struct Bullet {
    /// Area de colision y posicion en pantalla.
    pub rect: Rect,
    /// Velocidad vertical en pixeles por segundo.
    pub speed: f32,
    /// Marca para eliminar la entidad cuando sale de pantalla o colisiona.
    pub active: bool,
}

/// Objetivo que desciende desde la parte superior.
pub struct Target {
    /// Area de colision y posicion en pantalla.
    pub rect: Rect,
    /// Marca para eliminar la entidad cuando sale de pantalla o colisiona.
    pub active: bool,
    /// Variante de movimiento, aspecto y puntuacion del objetivo.
    pub target_type: TargetType,
    pub(super) origin_x: f32,
    pub(super) elapsed: f32,
}

#[derive(Clone, Copy)]
pub enum TargetType {
    /// Objetivo rojo que desciende a velocidad base.
    Standard,
    /// Objetivo naranja mas pequeno, rapido y de mayor puntuacion.
    Fast,
    /// Objetivo morado/azul que oscila horizontalmente al descender.
    ZigZag,
}

impl TargetType {
    /// Devuelve el ancho y alto del objetivo en pixeles.
    pub fn size(self) -> f32 {
        match self {
            Self::Standard | Self::ZigZag => 35.0,
            Self::Fast => 20.0,
        }
    }

    /// Devuelve el multiplicador aplicado a la velocidad base.
    pub fn speed_multiplier(self) -> f32 {
        match self {
            Self::Fast => 2.0,
            Self::Standard | Self::ZigZag => 1.0,
        }
    }

    /// Devuelve los puntos otorgados al destruir este tipo de objetivo.
    pub fn points(self) -> u32 {
        match self {
            Self::Fast => 200,
            Self::Standard | Self::ZigZag => 100,
        }
    }

    /// Devuelve el color usado para dibujar este tipo de objetivo.
    pub fn color(self) -> Color {
        match self {
            Self::Standard => RED,
            Self::Fast => ORANGE,
            Self::ZigZag => Color::new(0.35, 0.2, 0.9, 1.0),
        }
    }

    /// Indica si el tipo utiliza movimiento oscilatorio horizontal.
    pub fn is_zigzag(self) -> bool {
        matches!(self, Self::ZigZag)
    }
}
