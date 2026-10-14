use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game00".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    game00::run().await;
}