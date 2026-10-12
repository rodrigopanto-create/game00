//! API publica del juego.
//!
//! Este crate expone la funcion `run` que inicia el loop principal.

pub mod entities;
pub mod game;

pub use game::run;
