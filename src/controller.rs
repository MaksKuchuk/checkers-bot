use crate::{game::game_object::GameObject, gui::Renderer};
use macroquad::prelude::{
    is_mouse_button_pressed, is_mouse_button_released, mouse_position, MouseButton,
};

pub struct Controller {
    game: GameObject,
    handeled_checker_pos: Option<(u32, u32)>,
}

impl Controller {
    pub fn new() -> Controller {
        let game = GameObject::new((8, 8));
        Controller {
            game,
            handeled_checker_pos: None,
        }
    }

    pub async fn run_player2player_game(&mut self) {
        loop {
            self.take_checker();
            self.place_checker();

            dbg!(self.game.is_win());

            Renderer::update(&self.game, self.handeled_checker_pos, mouse_position()).await;
        }
    }

    pub async fn run_player2bot_game(&mut self) {
        loop {
            self.take_checker();
            self.place_checker();

            Renderer::update(&self.game, self.handeled_checker_pos, mouse_position()).await;
        }
    }

    pub async fn run_bot2bot_game(&self) {
        loop {
            Renderer::update(&self.game, None, mouse_position()).await;
        }
    }

    fn take_checker(&mut self) {
        if !is_mouse_button_pressed(MouseButton::Left) {
            return;
        }

        let pos = match self.get_cell_by_pixel(mouse_position()) {
            Some(v) => v,
            None => return,
        };

        let steps = self.game.get_loc_possible_steps();

        for p in steps {
            if pos == p.0 {
                self.handeled_checker_pos = Some(pos);
                break;
            }
        }
    }

    fn place_checker(&mut self) {
        if !is_mouse_button_released(MouseButton::Left) {
            return;
        }

        let h_pos = match self.handeled_checker_pos {
            Some(v) => v,
            None => return,
        };

        let pos = match self.get_cell_by_pixel(mouse_position()) {
            Some(v) => v,
            None => return,
        };

        let steps = self.game.get_loc_possible_steps();

        if steps.contains(&(h_pos, pos)) {
            self.game.make_step(h_pos, pos);
        }
        self.handeled_checker_pos = None;
    }

    fn get_cell_by_pixel(&self, pos: (f32, f32)) -> Option<(u32, u32)> {
        let h_cell = Renderer::get_cell_by_pixel(&self.game, pos);

        if h_cell.0 < 0
            || h_cell.1 < 0
            || h_cell.0 as usize >= self.game.get_board_size().0
            || h_cell.1 as usize >= self.game.get_board_size().1
        {
            None
        } else {
            Some((
                h_cell.0 as u32,
                (self.game.get_board_size().1 as i32 - h_cell.1 - 1) as u32,
            ))
        }
    }
}
