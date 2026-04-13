use macroquad::prelude::*;
use crate::ui::assets::load_assets;
use crate::ui::render::draw_board;

pub async fn run_ui() {
    let assets = load_assets().await;
    
    loop {
        draw_board(&assets);

        next_frame().await;
    }
}