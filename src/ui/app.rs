use macroquad::prelude::*;

pub async fn run_ui() {
    loop {
        clear_background(LIGHTGRAY);

        draw_text("Pond UI", 20.0, 40.0, 40.0, BLACK);

        next_frame().await;
    }
}