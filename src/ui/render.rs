use crate::ui::assets::Assets;
use macroquad::prelude::*;
use crate::environments::pond::{Cell, Pond};
use std::process::id;
use crate::traits::Env;

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
    draw_text_ex("Pond", 20.0, 40.0, TextParams {
        font: Some(&assets.font),
        font_size: 40,
        color: YELLOW,
        ..Default::default()
    },);

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

        // Pièces
        let index = losange.id as usize;
        let cell_index = pond.get_cell_index(index);
        draw_piece(assets, cell_index, losange.screen_x-45.0, losange.screen_y-45.0, cell_size-10.0);
    }
}

// Pièces
pub fn draw_piece(assets: &Assets, cell: Cell, x: f32, y:f32, size: f32){

    match cell {
        Cell::Empty => {},
        Cell::Egg(owner) => {
            match owner {
                0 => {draw_textured_rect(&assets.white_egg, x, y, size, size)}
                1 => {draw_textured_rect(&assets.dark_egg, x, y, size, size)}
                _ => {}
            }
        },
        Cell::Tadpole(owner) => {
            match owner {
                0 => {draw_textured_rect(&assets.white_tadpole, x, y, size, size)}
                1 => {draw_textured_rect(&assets.dark_tadpole, x, y, size, size)}
                _ => {}
            }
        },
        Cell::Frog(owner) => {
            match owner {
                0 => {draw_textured_rect(&assets.white_frog, x, y, size, size)}
                1 => {draw_textured_rect(&assets.dark_frog, x, y, size, size)}
                _ => {}
            }
        }
    }
}

// HUD
pub fn draw_hud(assets: &Assets,pond: &Pond){
    let padding_x = 20.0;
    let padding_y = 12.0;

    // Tour player
    let player = (pond.current_player() + 1).to_string();
    let player_texte = format!("Au tour du joueur {}", player);
    let dims_player_texte = measure_text(&player_texte, Some(&assets.font), 40, 1.0);

    let text_y_player = 60.0;
    let text_x_player = screen_width() / 2.0 - dims_player_texte.width / 2.0;

    let panel_w_player = dims_player_texte.width + padding_x * 2.0;
    let panel_h_player = dims_player_texte.height + padding_y * 2.0;
    let panel_x_player = screen_width() / 2.0 - panel_w_player / 2.0;
    let panel_y_player = text_y_player - 40.0;

    // Fond player
    draw_rectangle(
        panel_x_player,
        panel_y_player,
        panel_w_player,
        panel_h_player,
        BLACK,
    );

    // Texte player
    draw_text_ex(&player_texte, text_x_player , text_y_player, TextParams {
        font: Some(&assets.font),
        font_size: 40,
        color: WHITE,
        ..Default::default()
    },);

    // Scores
    let score_player1 = pond.get_player_score(0);
    let score_player2 = pond.get_player_score(1);

    let score_player1_texte = format!("Score Joueur 1 : {}", score_player1);
    let score_player2_texte = format!("Score Joueur 2 : {}", score_player2);

    let dims_score1_texte = measure_text(&score_player1_texte, Some(&assets.font), 40, 1.0);
    let dims_score2_texte = measure_text(&score_player2_texte, Some(&assets.font), 40, 1.0);

    let text_y_score = 150.0;

    // Joueur 1 gauche
    let text_x_score1 = 40.0;

    let panel_w_score1 = dims_score1_texte.width + padding_x * 2.0;
    let panel_h_score1 = dims_score1_texte.height + padding_y * 2.0;
    let panel_x_score1 = text_x_score1 - padding_x;
    let panel_y_score = text_y_score - dims_score1_texte.height - padding_y;

    // Fond score 1
    draw_rectangle(
        panel_x_score1,
        panel_y_score,
        panel_w_score1,
        panel_h_score1,
        BLACK,
    );

    // Texte score 1
    draw_text_ex(&score_player1_texte, text_x_score1 , text_y_score, TextParams {
        font: Some(&assets.font),
        font_size: 40,
        color: WHITE,
        ..Default::default()
    },);

    // Joueur 2 droite
    let text_x_score2 = screen_width() - (dims_score2_texte.width + 40.0);

    let panel_w_score2 = dims_score2_texte.width + padding_x * 2.0;
    let panel_h_score2 = dims_score2_texte.height + padding_y * 2.0;
    let panel_x_score2 = text_x_score2 - padding_x;

    // Fond score 2
    draw_rectangle(
        panel_x_score2,
        panel_y_score,
        panel_w_score2,
        panel_h_score2,
        BLACK,
    );

    // Texte score 2
    draw_text_ex(&score_player2_texte, text_x_score2, text_y_score, TextParams {
        font: Some(&assets.font),
        font_size: 40,
        color: WHITE,
        ..Default::default()
    },);

    // Pièces collectées
    let cell_size: f32 = 60.0;
    let spacing: f32 = 65.0;

    let collected_player1 = pond.get_collected_pieces(0);
    let collected_player2 = pond.get_collected_pieces(1);

    let screen_x_joueur1 = 30.0;
    let screen_y_joueurs = 200.0;
    let screen_x_joueur2 = screen_width() - 30.0 - cell_size;

    for (i, piece) in collected_player1.iter().enumerate() {
        let col = i % 5;
        let row = i / 5;

        let x = screen_x_joueur1 + col as f32 * spacing;
        let y = screen_y_joueurs + row as f32 * spacing;

        draw_piece(assets, *piece, x, y, cell_size);
    }

    for (i, piece) in collected_player2.iter().enumerate() {
        let col = i % 5;
        let row = i / 5;

        let x = screen_x_joueur2 - col as f32 * spacing;
        let y = screen_y_joueurs + row as f32 * spacing;

        draw_piece(assets, *piece, x, y, cell_size);
    }
}