use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::ui::assets::load_assets;
use crate::ui::render::{draw_board};
use crate::ui::input::{losange_clicked, losange_hovered};

pub async fn run_ui() {
    let assets = load_assets().await;
    let mut selected_losange: Option<i32> = None;
    
    loop {
        set_window_size(1280, 720);
        draw_board(&assets);

        let board_x= screen_width() / 2.0 - 250.0;
        let board_y= screen_height() / 2.0 - 250.0;
        let cell_size = 100.0;
        if let Some(id) = losange_clicked(board_x, board_y, cell_size){
            selected_losange = Some(id);
            println!("Selected losange clicked {}", id);
        }
        
        losange_hovered(board_x, board_y, cell_size);
        next_frame().await;
    }
}