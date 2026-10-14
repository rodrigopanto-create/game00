use super::highscores::{HighScores, InitialsEntry, INITIALS_LEN, MAX_ENTRIES};
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
        let restart_msg = "PRESS FIRE TO CONTINUE";
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

fn draw_centered(text: &str, y: f32, font_size: u16, color: Color) {
    let size = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        (screen_width() / 2.0 - size.width / 2.0).floor(),
        y.floor(),
        font_size as f32,
        color,
    );
}

pub(super) fn draw_new_record(entry: &InitialsEntry, score: u32, blink_timer: f32) {
    clear_background(BLACK);

    let blink_on = (blink_timer * 3.0) as i32 % 2 == 0;
    let title_color = if blink_on { YELLOW } else { ORANGE };
    draw_centered("NEW RECORD!", screen_height() * 0.18, 50, title_color);
    draw_centered(&format!("SCORE {score}"), screen_height() * 0.18 + 45.0, 25, WHITE);
    draw_centered("ENTER YOUR INITIALS", screen_height() * 0.38, 22, SKYBLUE);

    let slot_w = 60.0;
    let gap = 20.0;
    let total_w = INITIALS_LEN as f32 * slot_w + (INITIALS_LEN - 1) as f32 * gap;
    let start_x = screen_width() / 2.0 - total_w / 2.0;
    let base_y = screen_height() * 0.55;

    for (i, &letter) in entry.letters().iter().enumerate() {
        let x = start_x + i as f32 * (slot_w + gap);
        let selected = i == entry.cursor();
        let confirmed = i < entry.cursor();
        let color = if selected {
            YELLOW
        } else if confirmed {
            GREEN
        } else {
            DARKGRAY
        };

        if !selected || blink_on {
            let ch = (letter as char).to_string();
            let size = measure_text(&ch, None, 60, 1.0);
            draw_text(
                &ch,
                (x + slot_w / 2.0 - size.width / 2.0).floor(),
                base_y,
                60.0,
                color,
            );
        }
        draw_rectangle(x, base_y + 10.0, slot_w, 6.0, color);

        if selected {
            let cx = x + slot_w / 2.0;
            draw_triangle(
                vec2(cx, base_y - 70.0),
                vec2(cx - 10.0, base_y - 58.0),
                vec2(cx + 10.0, base_y - 58.0),
                YELLOW,
            );
            draw_triangle(
                vec2(cx, base_y + 40.0),
                vec2(cx - 10.0, base_y + 28.0),
                vec2(cx + 10.0, base_y + 28.0),
                YELLOW,
            );
        }
    }

    draw_centered(
        "< > CHANGE LETTER   FIRE CONFIRM",
        screen_height() * 0.55 + 80.0,
        18,
        GRAY,
    );
}

pub(super) fn draw_high_scores(scores: &HighScores, highlight: Option<usize>, blink_timer: f32) {
    clear_background(BLACK);

    draw_centered("HIGH SCORES", 60.0, 45, YELLOW);

    let row_h = 30.0;
    let top = 110.0;
    let blink_on = (blink_timer * 4.0) as i32 % 2 == 0;
    let entries = scores.entries();

    for rank in 0..MAX_ENTRIES {
        let y = top + rank as f32 * row_h;
        let entry = entries.get(rank);
        let is_new = highlight == Some(rank);
        if is_new && !blink_on {
            continue;
        }

        let color = match (is_new, rank) {
            (true, _) => YELLOW,
            (false, 0) => GOLD,
            (false, 1..=2) => ORANGE,
            _ => WHITE,
        };
        let line = match entry {
            Some(entry) => format!("{:>2}. {}  {:>7}", rank + 1, entry.initials_str(), entry.score),
            None => format!("{:>2}. ---  {:>7}", rank + 1, "-----"),
        };
        draw_centered(&line, y, 26, if entry.is_some() { color } else { DARKGRAY });
    }

    if (blink_timer * 2.5) as i32 % 2 == 0 {
        draw_centered(
            "PRESS FIRE TO RETURN",
            top + MAX_ENTRIES as f32 * row_h + 15.0,
            20,
            GREEN,
        );
    }
}
