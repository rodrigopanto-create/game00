use macroquad::prelude::*;

pub mod entities;
pub mod game;

pub use game::run;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game00".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    run().await;
}