use crate::ui::assets::load_assets;
use crate::ui::input::{losange_clicked, losange_hovered};
use crate::ui::render::{board_positions, draw_board};
use crate::traits::{Env, Action};
use crate::environments::pond::Pond;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

pub async fn run_ui() {
    let assets = load_assets().await;
    let mut selected_losange: Option<i32> = None;
    let mut pond = Pond::new();
    set_window_size(1280, 720);

    loop {
        let board_x = screen_width() / 2.0 - 250.0;
        let board_y = screen_height() / 2.0 - 250.0;
        let cell_size = 100.0;

        if let Some(id) = losange_clicked(board_x, board_y, cell_size)
            && selected_losange == None
        {
            selected_losange = Some(id);
            println!("Selected losange clicked {}", id);
        }

        draw_board(&assets);

        let losanges = board_positions(board_x, board_y, cell_size);
        for losange in &losanges {
            if selected_losange == Some(losange.id) {
                draw_rectangle(
                    losange.screen_x - 40.0,
                    losange.screen_y - 40.0,
                    cell_size - 20.0,
                    cell_size - 20.0,
                    Color::from_rgba(0, 255, 0, 128),
                );
            }
        }

        // Pas de sélection = hover possible
        if selected_losange.is_none() {
            losange_hovered(board_x, board_y, cell_size);
        }

        // Placer un oeuf
        if selected_losange.is_some() {
            if root_ui().button(vec2(30.0, 70.0), "Placer un oeuf") {
                if let Some(id) = selected_losange {
                    let action = id as Action;
                    let actions = pond.legal_action();
                    if actions.contains(&action) {
                        pond.step(action);
                        selected_losange = None;
                    }
                }
            }

            // Annuler la sélection
            if root_ui().button(vec2(30.0, 110.0), "Annuler") {
                selected_losange = None;
            }
        }

        next_frame().await;
    }
}
