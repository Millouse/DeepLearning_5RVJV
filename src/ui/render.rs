use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::ui::assets::Assets;

pub fn draw_background(assets: &Assets){
    draw_texture(&assets.background, 0.0, 0.0, WHITE);
}
pub fn draw_board(assets: &Assets) {
    draw_background(assets);

    draw_text("Pond UI", 20.0, 40.0, 40.0, WHITE);

    draw_rectangle(screen_width() / 2.0 - 200.0, 150.0, 400.0, 100.0, WHITE);
    draw_rectangle(screen_width() / 2.0 - 200.0, 250.0, 400.0, 100.0, WHITE);
    draw_rectangle(screen_width() / 2.0 - 200.0, 350.0, 400.0, 100.0, WHITE);
    draw_rectangle(screen_width() / 2.0 - 200.0, 450.0, 400.0, 100.0, WHITE);
}