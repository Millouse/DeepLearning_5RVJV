use crate::ui::render::board_positions;
use macroquad::prelude::*;
use macroquad::ui::widgets;
use macroquad::ui::widgets::Button;

pub fn losange_clicked(board_x: f32, board_y: f32, cell_size: f32) -> Option<i32> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    } else {
        let (mouse_x, mouse_y) = mouse_position();
        let losanges = board_positions(board_x, board_y, cell_size);

        for losange in losanges {
            let delta_x = mouse_x - losange.screen_x;
            let delta_y = mouse_y - losange.screen_y;
            let distance = delta_x * delta_x + delta_y * delta_y;

            if distance <= 2500.0 {
                return Some(losange.id);
            }
        }

        None
    }
}

pub fn losange_hovered(board_x: f32, board_y: f32, cell_size: f32) {
    let losanges = board_positions(board_x, board_y, cell_size);

    let (mouse_x, mouse_y) = mouse_position();
    for losange in losanges {
        let delta_x = mouse_x - losange.screen_x;
        let delta_y = mouse_y - losange.screen_y;
        let distance = delta_x * delta_x + delta_y * delta_y;

        if distance <= 2500.0 {
            draw_rectangle(
                losange.screen_x - 40.0,
                losange.screen_y - 40.0,
                cell_size - 20.0,
                cell_size - 20.0,
                Color::from_rgba(0, 255, 255, 128),
            );
        }
    }
}
