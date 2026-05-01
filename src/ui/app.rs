use crate::ui::assets::load_assets;
use crate::ui::input::{losange_clicked, losange_hovered};
use crate::ui::render::{board_positions, draw_board, draw_hud, custom_button, draw_background};
use crate::traits::{Env, Action, Agent};
use crate::environments::pond::{Cell, Pond};
use crate::agents::random::RandomAgent;
use crate::agents::random_rollout::RandomRolloutAgent;
use crate::agents::mcts::MCTS;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

#[derive(Clone, Copy, PartialEq)]
enum AgentType {
    Human,
    Random,
    Rollout,
    Mcts,
}

enum GamePhase {
    ChoosePlayer1,
    ChoosePlayer2,
    Playing,
}

pub async fn run_ui() {
    let assets = load_assets().await;
    let mut selected_losange: Option<i32> = None;
    let mut pond = Pond::new();
    let mut ai_random = RandomAgent;
    let mut ai_rollout = RandomRolloutAgent::new(100);
    let mut ai_mcts = MCTS::new(1000, 1.41);
    let mut ai_delay: f32 = 0.0;
    let ai_pause: f32 = 0.5;
    let mut phase = GamePhase::ChoosePlayer1;
    let mut agents: [AgentType; 2] = [AgentType::Human, AgentType::Random];
    set_window_size(1280, 720);

    loop {
        let board_x = screen_width() / 2.0 - 250.0;
        let board_y = screen_height() / 2.0 - 250.0;
        let cell_size = 100.0;
        let button_w = 120.0;
        let button_h = 40.0;
        let dims_pond = measure_text("POND", Some(&assets.font), 150, 1.0);

        match phase {
            GamePhase::ChoosePlayer1 => {
                draw_background(&assets);

                let dims_joueur1 = measure_text("Joueur 1 :", Some(&assets.font), 36, 1.0);

                draw_text_ex("POND", screen_width() / 2.0 - dims_pond.width / 2.0, 150.0, TextParams {
                    font: Some(&assets.font),
                    font_size: 150,
                    color: YELLOW,
                    ..Default::default()
                },);

                draw_text_ex("Joueur 1 :", screen_width() / 2.0 - dims_joueur1.width / 2.0, 200.0, TextParams {
                    font: Some(&assets.font),
                    font_size: 36,
                    color: WHITE,
                    ..Default::default()
                },);

                if custom_button(&assets, "Humain", screen_width() / 2.0 - button_w / 2.0, 240.0, button_w, button_h) {
                    agents[0] = AgentType::Human;
                    phase = GamePhase::ChoosePlayer2;
                }
                if custom_button(&assets, "Random", screen_width() / 2.0 - button_w / 2.0, 300.0, button_w, button_h) {
                    agents[0] = AgentType::Random;
                    phase = GamePhase::ChoosePlayer2;
                }
                if custom_button(&assets, "Rollout", screen_width() / 2.0 - button_w / 2.0, 360.0, button_w, button_h) {
                    agents[0] = AgentType::Rollout;
                    phase = GamePhase::ChoosePlayer2;
                }
                if custom_button(&assets, "MCTS", screen_width() / 2.0 - button_w / 2.0, 420.0, button_w, button_h) {
                    agents[0] = AgentType::Mcts;
                    phase = GamePhase::ChoosePlayer2;
                }
                next_frame().await;
                continue;
            }
            GamePhase::ChoosePlayer2 => {
                draw_background(&assets);

                let dims_joueur2 = measure_text("Joueur 2 :", Some(&assets.font), 36, 1.0);

                draw_text_ex("POND", screen_width() / 2.0 - dims_pond.width / 2.0, 150.0, TextParams {
                    font: Some(&assets.font),
                    font_size: 150,
                    color: YELLOW,
                    ..Default::default()
                },);

                draw_text_ex("Joueur 2 :", screen_width() / 2.0 - dims_joueur2.width / 2.0, 200.0, TextParams {
                    font: Some(&assets.font),
                    font_size: 36,
                    color: WHITE,
                    ..Default::default()
                },);
                if custom_button(&assets, "Humain", screen_width() / 2.0 - button_w / 2.0, 240.0, button_w, button_h) {
                    agents[1] = AgentType::Human;
                    phase = GamePhase::Playing;
                }
                if custom_button(&assets, "Random", screen_width() / 2.0 - button_w / 2.0, 300.0, button_w, button_h) {
                    agents[1] = AgentType::Random;
                    phase = GamePhase::Playing;
                }
                if custom_button(&assets, "Rollout", screen_width() / 2.0 - button_w / 2.0, 360.0, button_w, button_h) {
                    agents[1] = AgentType::Rollout;
                    phase = GamePhase::Playing;
                }
                if custom_button(&assets, "MCTS", screen_width() / 2.0 - button_w / 2.0, 420.0, button_w, button_h) {
                    agents[1] = AgentType::Mcts;
                    phase = GamePhase::Playing;
                }
                next_frame().await;
                continue;
            }
            GamePhase::Playing => {}
        }

        let current = pond.current_player();
        let current_agent = agents[current];

        if !pond.is_game_over() && current_agent != AgentType::Human {
            ai_delay += get_frame_time();
            if ai_delay >= ai_pause {
                let obs = pond.to_observation();
                let legal = pond.legal_action();
                if !legal.is_empty() {
                    let action = match current_agent {
                        AgentType::Random => ai_random.select_action(&obs, legal, Some(&pond as &dyn Env)),
                        AgentType::Rollout => ai_rollout.select_action(&obs, legal, Some(&pond as &dyn Env)),
                        AgentType::Mcts => ai_mcts.select_action(&obs, legal, Some(&pond as &dyn Env)),
                        _ => unreachable!(),
                    };
                    pond.step(action);
                }
                ai_delay = 0.0;
            }
        } else {
            ai_delay = 0.0;
        }

        if !pond.is_game_over() && current_agent == AgentType::Human {
            if let Some(id) = losange_clicked(board_x, board_y, cell_size) {
                handle_human_click(&mut pond, &mut selected_losange, id, current);
            }
        }

        draw_board(&assets, &pond);
        draw_hud(&assets, &pond);

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

        if selected_losange.is_none()
            && !pond.is_game_over()
            && current_agent == AgentType::Human
        {
            losange_hovered(board_x, board_y, cell_size);
        }

        if selected_losange.is_some() {
            let dims_annuler_texte = measure_text("Annuler", Some(&assets.font), 36, 1.0);

            if custom_button(&assets, "Annuler", screen_width() / 2.0 - dims_annuler_texte.width / 2.0, 650.0, button_w, button_h) {
                selected_losange = None;
            }
        }

        if pond.is_game_over() {
            let winner_text = match (pond.get_player_score(0), pond.get_player_score(1)) {
                (a, b) if a > b => "Le joueur 1 gagne !",
                (a, b) if b > a => "Le joueur 2 gagne !",
                _ => "Egalité !",
            };

            // Transparent
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.45),
            );

            // Fond
            let panel_w = 350.0;
            let panel_h = 150.0;
            let panel_x = screen_width() / 2.0 - panel_w / 2.0;
            let panel_y = screen_height() / 2.0 - panel_h / 2.0;

            draw_rectangle(
                panel_x,
                panel_y,
                panel_w,
                panel_h,
                BLACK,
            );

            // Texte gagnant
            let dims_winner_texte = measure_text(winner_text, Some(&assets.font), 36, 1.0);

            draw_text_ex(
                winner_text,
                screen_width() / 2.0 - dims_winner_texte.width / 2.0,
                panel_y + 50.0, TextParams {
                    font: Some(&assets.font),
                    font_size: 36,
                    color: YELLOW,
                    ..Default::default()
                },
            );

            // Rejouer
            let button_w = 120.0;
            let button_h = 40.0;
            let button_x = screen_width() / 2.0 - button_w / 2.0;
            let button_y = panel_y + 80.0;

            if custom_button(&assets, "Rejouer", button_x, button_y, button_w, button_h) {
                pond.reset();
                selected_losange = None;
                phase = GamePhase::ChoosePlayer1;
            }
        }

        next_frame().await;
    }
}

fn handle_human_click(pond: &mut Pond, selected: &mut Option<i32>, clicked_id: i32, player: usize) {
    let legal = pond.legal_action();
    let idx = clicked_id as usize;

    match *selected {
        None => {
            let cell = pond.get_cell_index(idx);
            match cell {
                Cell::Empty => {
                    let action: Action = idx;
                    if legal.contains(&action) {
                        pond.step(action);
                    }
                }
                Cell::Tadpole(o) | Cell::Frog(o) if o == player => {
                    *selected = Some(clicked_id);
                }
                _ => {}
            }
        }
        Some(src_id) => {
            if src_id == clicked_id {
                *selected = None;
                return;
            }
            let src = src_id as usize;
            let dst = idx;
            let src_cell = pond.get_cell_index(src);
            let action: Option<Action> = match src_cell {
                Cell::Tadpole(o) if o == player => Some(16 + src * 16 + dst),
                Cell::Frog(o)    if o == player => Some(16 + 256 + src * 16 + dst),
                _ => None,
            };

            if let Some(a) = action {
                if legal.contains(&a) {
                    pond.step(a);
                    *selected = None;
                }
            } else {
                *selected = None;
            }
        }
    }
}