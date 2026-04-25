use crate::ui::assets::Assets;
use macroquad::prelude::*;
use crate::environments::pond::{Cell, Pond};
use std::process::id;

pub struct BoardPosition {
    pub id: i32,
    pub ligne: i32,
    pub colonne: i32,
    pub screen_x: f32,
    pub screen_y: f32,
}

pub fn draw_background(assets: &Assets) {
    draw_texture(&assets.background, 0.0, 0.0, WHITE);
}

// Positions des losanges
pub fn board_positions(board_x: f32, board_y: f32, cell_size: f32) -> Vec<BoardPosition> {
    let mut positions = Vec::new();
    let mut id = 0;

    for colonne in 0..4 {
        for ligne in 0..4 {
            let screen_x = board_x + cell_size + ligne as f32 * cell_size;
            let screen_y = board_y + cell_size + colonne as f32 * cell_size;

            positions.push(BoardPosition {
                id,
                ligne,
                colonne,
                screen_x,
                screen_y,
            });

            id += 1;
        }
    }
    positions
}

// Textures bois
pub fn draw_textured_rect(texture: &Texture2D, x: f32, y: f32, w: f32, h: f32) {
    draw_texture_ex(
        texture,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );
}

pub fn draw_board(assets: &Assets, pond: &Pond) {
    draw_background(assets);
    draw_text("Pond UI", 20.0, 40.0, 40.0, WHITE);

    let cell_size: f32 = 100.0;
    let board_x = screen_width() / 2.0 - 250.0;
    let board_y = screen_height() / 2.0 - 250.0;

    // Plateau
    draw_textured_rect(
        &assets.dark_wood,
        board_x - 10.0,
        board_y - 10.0,
        cell_size * 5.0 + 20.0,
        cell_size * 5.0 + 20.0,
    );
    draw_textured_rect(
        &assets.white_wood,
        board_x,
        board_y,
        cell_size * 5.0,
        cell_size * 5.0,
    );

    // Lignes verticales
    for ligne_v in 0..4 {
        let x_ligne_v = board_x + 100.0 + ligne_v as f32 * cell_size;
        let y1_ligne_v = board_y;
        let y2_ligne_v = board_y + 500.0;

        draw_line(x_ligne_v, y1_ligne_v, x_ligne_v, y2_ligne_v, 2.0, DARKBROWN);
    }

    // Lignes horizontales
    for ligne_h in 0..4 {
        let x1_ligne_h = board_x;
        let x2_ligne_h = board_x + 500.0;
        let y_ligne_h = board_y + 100.0 + ligne_h as f32 * cell_size;

        draw_line(x1_ligne_h, y_ligne_h, x2_ligne_h, y_ligne_h, 2.0, DARKBROWN);
    }

    // Dessin des losanges
    let losanges = board_positions(board_x, board_y, cell_size);

    for losange in losanges {
        draw_poly(losange.screen_x, losange.screen_y, 4, 25.0, 0.0, DARKBROWN);
        let index = losange.id as usize;
        let cell_index = pond.get_cell_index(index);
        draw_piece(assets, cell_index, losange.screen_x, losange.screen_y, cell_size, cell_size);
    }
}

pub fn draw_piece(assets: &Assets, cell: Cell, x: f32, y:f32, w: f32, h:f32){
    match cell {
        Cell::Empty => {},
        Cell::Egg(owner) => {
            match owner {
                0 => {draw_textured_rect(&assets.white_egg, x-50.0, y-50.0, w-10.0, h-10.0)}
                1 => {draw_textured_rect(&assets.dark_egg, x-50.0, y-50.0, w-10.0, h-10.0)}
                _ => {}
            }
        },
        Cell::Tadpole(owner) => {
            match owner {
                0 => {draw_textured_rect(&assets.white_tadpole, x-50.0, y-50.0, w-10.0, h-10.0)}
                1 => {draw_textured_rect(&assets.dark_tadpole, x-50.0, y-50.0, w-10.0, h-10.0)}
                _ => {}
            }
        },
        Cell::Frog(owner) => {
            match owner {
                0 => {draw_textured_rect(&assets.white_frog, x-50.0, y-50.0, w-10.0, h-10.0)}
                1 => {draw_textured_rect(&assets.dark_frog, x-50.0, y-50.0, w-10.0, h-10.0)}
                _ => {}
            }
        }
    }
}
