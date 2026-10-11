use crate::entities::{Bullet, Target};
use macroquad::prelude::*;

/// Ejecuta el loop principal del juego.
pub async fn run() {
    let mut player_rect = Rect::new(screen_width() / 2.0, screen_height() - 60.0, 40.0, 40.0);
    let player_speed = 400.0;
    let ship_texture = create_ship_texture();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut targets: Vec<Target> = Vec::new();

    let mut score = 0;
    let mut spawn_timer = 0.0;

    loop {
        let delta = get_frame_time();
        spawn_timer += delta;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player_rect.x += player_speed * delta;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player_rect.x -= player_speed * delta;
        }

        player_rect.x = player_rect.x.clamp(0.0, screen_width() - player_rect.w);

        if is_key_pressed(KeyCode::Space) {
            bullets.push(Bullet {
                rect: Rect::new(player_rect.x + player_rect.w / 2.0 - 4.0, player_rect.y, 8.0, 16.0),
                speed: 600.0,
                active: true,
            });
        }

        if spawn_timer > 1.0 {
            spawn_timer = 0.0;
            let random_x = rand::gen_range(20.0, screen_width() - 60.0);
            targets.push(Target {
                rect: Rect::new(random_x, -40.0, 35.0, 35.0),
                active: true,
            });
        }

        for bullet in bullets.iter_mut() {
            bullet.rect.y -= bullet.speed * delta;
            if bullet.rect.y < -20.0 {
                bullet.active = false;
            }
        }

        for target in targets.iter_mut() {
            target.rect.y += 150.0 * delta;
            if target.rect.y > screen_height() {
                target.active = false;
            }
        }

        for bullet in bullets.iter_mut().filter(|bullet| bullet.active) {
            for target in targets.iter_mut().filter(|target| target.active) {
                if bullet.rect.overlaps(&target.rect) {
                    bullet.active = false;
                    target.active = false;
                    score += 100;
                }
            }
        }

        bullets.retain(|bullet| bullet.active);
        targets.retain(|target| target.active);

        clear_background(BLACK);

        draw_texture_ex(
            &ship_texture,
            player_rect.x,
            player_rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(player_rect.w, player_rect.h)),
                ..Default::default()
            },
        );
        for bullet in &bullets {
            draw_rectangle(bullet.rect.x, bullet.rect.y, bullet.rect.w, bullet.rect.h, YELLOW);
        }

        for target in &targets {
            draw_rectangle(target.rect.x, target.rect.y, target.rect.w, target.rect.h, RED);
        }

        draw_text(&format!("SCORE: {}", score), 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}

/// Crea una textura simple para el jugador sin depender de archivos externos.
fn create_ship_texture() -> Texture2D {
    let image = Image::gen_image_color(40, 40, Color::new(0.15, 0.85, 0.35, 1.0));
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}