use macroquad::{
    prelude::Color,
    shapes::{draw_circle, draw_circle_lines, draw_rectangle},
    window::{clear_background, next_frame, screen_height, screen_width},
};

use crate::game::{checker::Order, game_object::GameObject};

pub static CELL_SIZE: f32 = 50.;
pub static CHECKER_SIZE: i32 = 40;
pub static STEP_SIZE: i32 = 15;

static BACKGROUND_CL: Color = Color::new(0.974, 0.974, 0.974, 1.);

static BLACK_CELL_CL: Color = Color::new(0.772, 0.392, 0.203, 1.);
static WHITE_CELL_CL: Color = Color::new(0.913, 0.796, 0.647, 1.);

static STEP_CL: Color = Color::new(0., 0., 0., 1.);

static CHECKER_BLACK_CL: Color = Color::new(0.16, 0.16, 0.16, 1.);
static CHECKER_WHITE_CL: Color = Color::new(0.882, 0.854, 0.741, 1.);

struct Screen {}

impl Screen {
    pub fn get_screen_center() -> (f32, f32) {
        (screen_width() / 2., screen_height() / 2.)
    }
}

pub struct Renderer {}

impl Renderer {
    fn draw_board(game: &GameObject) {
        let start_pos = Self::get_start_position(game);

        for x in 0..game.get_board_size().0 {
            for y in 0..game.get_board_size().1 {
                draw_rectangle(
                    start_pos.0 + x as f32 * CELL_SIZE,
                    start_pos.1 + y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    if (x + y) % 2 != 0 {
                        BLACK_CELL_CL
                    } else {
                        WHITE_CELL_CL
                    },
                )
            }
        }
    }

    fn draw_checkers(game: &GameObject, handeled_pos: Option<(u32, u32)>) {
        let h_pos = match handeled_pos {
            Some(v) => (v.0 as i32, v.1 as i32),
            None => (-1, -1),
        };

        for x in 0..game.get_board_size().0 {
            for y in 0..game.get_board_size().1 {
                if (x as i32, y as i32) == h_pos {
                    continue;
                }

                Self::draw_checker_pos(game, (x as u32, y as u32));
            }
        }
    }

    fn draw_possible_steps(game: &GameObject, handeled_pos: Option<(u32, u32)>) {
        let checker_pos = match handeled_pos {
            Some(v) => v,
            None => return,
        };

        let steps = game.get_loc_possible_steps();

        for pos in steps {
            if checker_pos == pos.0 {
                Renderer::draw_circle_in_cell(
                    game,
                    (pos.1 .0 as i32, pos.1 .1 as i32),
                    STEP_SIZE,
                    STEP_CL,
                )
            }
        }
    }

    fn draw_mouse(game: &GameObject, handeled_checker_pos: Option<(u32, u32)>, pos: (f32, f32)) {
        let h_pos = match handeled_checker_pos {
            Some(v) => v,
            None => return,
        };

        let h_pos = (h_pos.0 as i32, h_pos.1 as i32);
        let col = match game.get_board_order_pos(h_pos) {
            Some(v) => match v {
                Order::WHITE => CHECKER_WHITE_CL,
                Order::BLACK => CHECKER_BLACK_CL,
            },
            _ => return,
        };

        if game.get_pos_is_king(h_pos) {
            draw_circle_lines(
                pos.0,
                pos.1,
                CHECKER_SIZE as f32 / 2.,
                CHECKER_SIZE as f32 / 4.,
                col,
            );
        } else {
            draw_circle(pos.0, pos.1, CHECKER_SIZE as f32 / 2., col);
        }
    }

    pub async fn update(
        game: &GameObject,
        handeled_checker_pos: Option<(u32, u32)>,
        mouse_pos: (f32, f32),
    ) {
        clear_background(BACKGROUND_CL);

        Self::draw_board(game);
        Self::draw_checkers(game, handeled_checker_pos);
        Self::draw_possible_steps(game, handeled_checker_pos);
        Self::draw_mouse(game, handeled_checker_pos, mouse_pos);

        next_frame().await
    }

    fn draw_checker_pos(game: &GameObject, pos: (u32, u32)) {
        let pos = (pos.0 as i32, pos.1 as i32);
        let col = match game.get_board_order_pos(pos) {
            Some(v) => match v {
                Order::WHITE => CHECKER_WHITE_CL,
                Order::BLACK => CHECKER_BLACK_CL,
            },
            _ => return,
        };

        if game.get_pos_is_king(pos) {
            Self::draw_circle_lines_in_cell(game, pos, CHECKER_SIZE, col)
        } else {
            Self::draw_circle_in_cell(game, pos, CHECKER_SIZE, col);
        }
    }

    fn get_start_position(game: &GameObject) -> (f32, f32) {
        let center = Screen::get_screen_center();
        (
            center.0 - CELL_SIZE * game.get_board_size().0 as f32 / 2.,
            center.1 - CELL_SIZE * game.get_board_size().1 as f32 / 2.,
        )
    }

    fn draw_circle_in_cell(game: &GameObject, pos: (i32, i32), size: i32, col: Color) {
        let start_pos = Self::get_start_position(game);
        let xpos = pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.;
        let ypos = (game.get_board_size().1 as i32 - pos.1 - 1) as f32 * CELL_SIZE + CELL_SIZE / 2.;
        draw_circle(
            start_pos.0 + xpos,
            start_pos.1 + ypos,
            size as f32 / 2.,
            col,
        );
    }
    fn draw_circle_lines_in_cell(game: &GameObject, pos: (i32, i32), size: i32, col: Color) {
        let start_pos = Self::get_start_position(game);
        let xpos = pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.;
        let ypos = (game.get_board_size().1 as i32 - pos.1 - 1) as f32 * CELL_SIZE + CELL_SIZE / 2.;
        draw_circle_lines(
            start_pos.0 + xpos,
            start_pos.1 + ypos,
            size as f32 / 2.,
            size as f32 / 4.,
            col,
        );
    }

    pub fn get_cell_by_pixel(game: &GameObject, pos: (f32, f32)) -> (i32, i32) {
        let (x, y) = pos;
        let board_start = Self::get_start_position(game);
        let x_p = ((x - board_start.0) / CELL_SIZE as f32) as i32;
        let y_p = ((y - board_start.1) / CELL_SIZE as f32) as i32;

        (x_p, y_p)
    }
}
