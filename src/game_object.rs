use crate::board::{Board, CellKind};
use crate::checker::Order;
use std::collections::HashSet;

pub struct GameObject {
    board: Board,
    order: Order,
    must_kill_checker: Option<(u32, u32)>,
    loc_possible_steps: HashSet<((u32, u32), (u32, u32))>,
}

impl GameObject {
    pub fn new(brd_size: (usize, usize)) -> GameObject {
        let mut game = GameObject {
            board: Board::new(brd_size),
            order: Order::WHITE,
            must_kill_checker: None,
            loc_possible_steps: HashSet::new(),
        };

        game.loc_possible_steps = game.get_possible_ways_for_all();
        game
    }

    pub fn show_board(&self) {
        self.board.print_board();
        for e in &self.loc_possible_steps {
            println!("({} {}) ({} {})", e.0 .0, e.0 .1, e.1 .0, e.1 .1);
        }
    }

    pub fn get_order(&self) -> Order {
        self.order.clone()
    }

    fn get_possible_ways_for_all(&self) -> HashSet<((u32, u32), (u32, u32))> {
        let mut steps: HashSet<((u32, u32), (u32, u32))> = HashSet::new();

        for x in 0..self.board.get_board_size().0 {
            for y in 0..self.board.get_board_size().1 {
                if let CellKind::Some(v) = self.board.checker_order((x as i32, y as i32)) {
                    if self.order != v {
                        continue;
                    };
                }

                let s = self
                    .board
                    .get_possible_steps((x as u32, y as u32), self.must_kill_checker);

                for item in s.0 {
                    steps.insert(((x as u32, y as u32), item));
                }
                for item in s.1 {
                    steps.insert(((x as u32, y as u32), item));
                }
            }
        }

        steps
    }

    pub fn get_loc_possible_steps(&self) -> &HashSet<((u32, u32), (u32, u32))> {
        &self.loc_possible_steps
    }

    pub fn make_step(&mut self, pos_from: (u32, u32), pos_to: (u32, u32)) -> bool {
        if !self.loc_possible_steps.contains(&(pos_from, pos_to)) {
            return false;
        }

        self.must_kill_checker = None;

        let (pos_from, pos_to) = (
            (pos_from.0 as i32, pos_from.1 as i32),
            (pos_to.0 as i32, pos_to.1 as i32),
        );

        self.board.move_pos_checker(
            (pos_from.0 as u32, pos_from.1 as u32),
            (pos_to.0 as u32, pos_to.1 as u32),
        );

        for i in 1..((pos_to.0 as i32 - pos_from.0 as i32).abs()) {
            let x_sign = (pos_to.0 - pos_from.0) / (pos_to.0 - pos_from.0).abs();
            let y_sign = (pos_to.1 - pos_from.1) / (pos_to.1 - pos_from.1).abs();

            if !self.board.is_pos_empty((
                (pos_from.0 + i * x_sign) as u32,
                (pos_from.1 + i * y_sign) as u32,
            )) {
                self.board.remove_pos_checker((
                    (pos_from.0 + i * x_sign) as u32,
                    (pos_from.1 + i * y_sign) as u32,
                ));

                println!("has killed");

                if !self
                    .board
                    .get_kill_steps((pos_to.0 as u32, pos_to.1 as u32))
                    .is_empty()
                {
                    println!("change");
                    self.change_order();
                    self.must_kill_checker = Some((pos_to.0 as u32, pos_to.1 as u32));
                }
            }
        }

        self.change_order();
        self.loc_possible_steps = self.get_possible_ways_for_all();
        true
    }

    fn change_order(&mut self) {
        self.order = if let Order::WHITE = self.order {
            Order::BLACK
        } else {
            Order::WHITE
        };
    }

    pub fn is_win(&self) -> Option<Order> {
        if self.loc_possible_steps.is_empty() {
            if self.order == Order::WHITE {
                Some(Order::BLACK)
            } else {
                Some(Order::WHITE)
            }
        } else {
            None
        }
    }
}