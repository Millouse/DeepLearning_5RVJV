use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::ui::assets::load_assets;
use crate::ui::render::draw_board;

pub async fn run_ui() {
    let assets = load_assets().await;
    
    loop {
        set_window_size(1280, 720);
        draw_board(&assets);

        next_frame().await;
    }
}