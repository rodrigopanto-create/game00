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
}