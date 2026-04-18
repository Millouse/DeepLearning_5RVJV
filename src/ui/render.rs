use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use crate::ui::assets::Assets;

pub fn draw_background(assets: &Assets){
    draw_texture(&assets.background, 0.0, 0.0, WHITE);
}

// Textures bois
pub fn draw_textured_rect(texture: &Texture2D, x: f32, y: f32, w: f32, h: f32) {
    draw_texture_ex( texture, x, y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );
}

pub fn draw_board(assets: &Assets) {
    draw_background(assets);
    draw_text("Pond UI", 20.0, 40.0, 40.0, WHITE);

    let cell_size:f32 = 100.0;
    let board_x = screen_width() / 2.0 - 250.0;
    let board_y = screen_height() / 2.0 - 250.0;

    // Plateau
    draw_textured_rect(&assets.dark_wood, board_x - 10.0, board_y - 10.0, cell_size * 5.0 + 20.0, cell_size * 5.0 + 20.0);
    draw_textured_rect(&assets.white_wood, board_x, board_y, cell_size * 5.0, cell_size * 5.0);

    // Lignes verticales
    for ligne_v in 0..4{
        let x_ligne_v = board_x + 100.0 + ligne_v as f32 * cell_size;
        let y1_ligne_v = board_y;
        let y2_ligne_v = board_y + 500.0;

        draw_line(x_ligne_v, y1_ligne_v, x_ligne_v, y2_ligne_v, 2.0, DARKBROWN);
    }

    // Lignes horizontales
    for ligne_h in 0..4{
        let x1_ligne_h = board_x;
        let x2_ligne_h = board_x + 500.0;
        let y_ligne_h = board_y + 100.0 + ligne_h as f32 * cell_size;

        draw_line(x1_ligne_h, y_ligne_h, x2_ligne_h, y_ligne_h, 2.0, DARKBROWN);
    }

    // Losanges
    for ligne in 0..4{
        let y_poly = board_y + 100.0 + ligne as f32 * cell_size;

        for colonne in 0..4{
            let x_poly = board_x + 100.0 + colonne as f32 * cell_size;

            draw_poly(x_poly, y_poly, 4, 25.0, 0.0, DARKBROWN);
        }
    }
}