use macroquad::prelude::*;
use crate::ui::assets::Assets;

pub fn draw_background(assets: &Assets){
    draw_texture(&assets.background, 0.0, 0.0, WHITE);
}
pub fn draw_board(assets: &Assets) {
    clear_background(LIGHTGRAY);
    draw_background(assets);
    draw_text("Pond UI", 20.0, 40.0, 40.0, WHITE);
}