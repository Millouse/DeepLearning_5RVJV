use crate::ui::assets::load_assets;
use crate::ui::input::{losange_clicked, losange_hovered};
use crate::ui::render::{board_positions, draw_board, draw_hud};
use crate::traits::{Env, Action, Agent};
use crate::environments::pond::{Cell, Pond};
use crate::agents::random::RandomAgent;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

/// Le joueur humain est toujours le joueur 0 ; l'IA random est le joueur 1.
const HUMAN: usize = 0;

pub async fn run_ui() {
    let assets = load_assets().await;
    let mut selected_losange: Option<i32> = None;
    let mut pond = Pond::new();
    let mut ai = RandomAgent;
    set_window_size(1280, 720);

    loop {
        let board_x = screen_width() / 2.0 - 250.0;
        let board_y = screen_height() / 2.0 - 250.0;
        let cell_size = 100.0;

        //Tour de l'IA random (joueur 1)
        if !pond.is_game_over() && pond.current_player() != HUMAN {
            let obs = pond.to_observation();
            let legal = pond.legal_action();
            if !legal.is_empty() {
                let action = ai.select_action(&obs, legal, Some(&pond as &dyn Env));
                pond.step(action);
            }
        }

        //Clic du joueur humain
        if !pond.is_game_over() && pond.current_player() == HUMAN {
            if let Some(id) = losange_clicked(board_x, board_y, cell_size) {
                handle_human_click(&mut pond, &mut selected_losange, id);
            }
        }

        //Affichage
        draw_board(&assets, &pond);
        draw_hud(&pond);

        // Surbrillance de la case sélectionnée
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

        // Hover quand rien n'est sélectionné et que c'est notre tour
        if selected_losange.is_none() && !pond.is_game_over() && pond.current_player() == HUMAN {
            losange_hovered(board_x, board_y, cell_size);
        }

        // Bouton Annuler
        if selected_losange.is_some() {
            if root_ui().button(vec2(30.0, 110.0), "Annuler") {
                selected_losange = None;
            }
        }

        // Fin de partie
        if pond.is_game_over() {
            let winner_text = match (pond.get_player_score(0), pond.get_player_score(1)) {
                (a, b) if a > b => "Le joueur 1 (humain) gagne !",
                (a, b) if b > a => "Le joueur 2 (IA) gagne !",
                _ => "Egalite",
            };
            draw_text(winner_text, 30.0, 160.0, 36.0, YELLOW);

            if root_ui().button(vec2(30.0, 200.0), "Rejouer") {
                pond.reset();
                selected_losange = None;
            }
        }

        next_frame().await;
    }
}


/// Règle : aucune sélection -> case vide = pose œuf, case avec ma pièce = sélection.
///         pièce sélectionnée -> clic case vide = tentative de déplacement,
///                               clic même pièce = annulation.
/// On valide toujours l'action contre `pond.legal_action()`.
fn handle_human_click(pond: &mut Pond, selected: &mut Option<i32>, clicked_id: i32) {
    let legal = pond.legal_action();
    let idx = clicked_id as usize;

    match *selected {
        None => {
            let cell = pond.get_cell_index(idx);
            match cell {
                Cell::Empty => {
                    // Pose oeuf
                    let action: Action = idx;
                    if legal.contains(&action) {
                        pond.step(action);
                    }
                }
                Cell::Tadpole(o) | Cell::Frog(o) if o == HUMAN => {
                    // Selection d'une de mes pieces
                    *selected = Some(clicked_id);
                }
                _ => {
                    // Piece adverse ou autre : on ignore
                }
            }
        }
        Some(src_id) => {
            if src_id == clicked_id {
                // Re-clic sur la meme case = annulation
                *selected = None;
                return;
            }
            let src = src_id as usize;
            let dst = idx;
            let src_cell = pond.get_cell_index(src);
            let action: Option<Action> = match src_cell {
                Cell::Tadpole(o) if o == HUMAN => Some(16 + src * 16 + dst),
                Cell::Frog(o)    if o == HUMAN => Some(16 + 256 + src * 16 + dst),
                _ => None,
            };

            if let Some(a) = action {
                if legal.contains(&a) {
                    pond.step(a);
                    *selected = None;
                } else {
                    // Coup illegal : on garde la selection pour reessayer ailleurs
                }
            } else {
                *selected = None;
            }
        }
    }
}
