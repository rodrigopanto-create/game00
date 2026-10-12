use macroquad::prelude::*;

pub(super) fn draw_start_screen(blink_timer: f32) {
    clear_background(BLACK);

    let title = "MACROQUAD SHOOTER";
    let title_size = measure_text(title, None, 40, 1.0);
    draw_text(
        title,
        screen_width() / 2.0 - title_size.width / 2.0,
        screen_height() / 3.0,
        40.0,
        YELLOW,
    );

    let credits = "1978 ATARI INC. / RUST EDITION";
    let credits_size = measure_text(credits, None, 18, 1.0);
    draw_text(
        credits,
        screen_width() / 2.0 - credits_size.width / 2.0,
        screen_height() / 3.0 + 40.0,
        18.0,
        DARKGRAY,
    );

    if (blink_timer * 2.5) as i32 % 2 == 0 {
        let start_msg = "PRESS SPACE TO START";
        let msg_size = measure_text(start_msg, None, 25, 1.0);
        draw_text(
            start_msg,
            screen_width() / 2.0 - msg_size.width / 2.0,
            screen_height() * 0.65,
            25.0,
            GREEN,
        );
    }
}

pub(super) fn draw_game_over(score: u32, blink_timer: f32) {
    clear_background(BLACK);

    let go_msg = "GAME OVER";
    let go_size = measure_text(go_msg, None, 50, 1.0);
    draw_text(
        go_msg,
        screen_width() / 2.0 - go_size.width / 2.0,
        screen_height() / 3.0,
        50.0,
        RED,
    );

    let final_score = format!("FINAL SCORE: {}", score);
    let score_size = measure_text(&final_score, None, 25, 1.0);
    draw_text(
        &final_score,
        screen_width() / 2.0 - score_size.width / 2.0,
        screen_height() / 3.0 + 50.0,
        25.0,
        WHITE,
    );

    if (blink_timer * 2.5) as i32 % 2 == 0 {
        let restart_msg = "PRESS SPACE TO RESTART";
        let restart_size = measure_text(restart_msg, None, 20, 1.0);
        draw_text(
            restart_msg,
            screen_width() / 2.0 - restart_size.width / 2.0,
            screen_height() * 0.65,
            20.0,
            YELLOW,
        );
    }
}
