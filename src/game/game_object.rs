use crate::game::board::{Board, CellKind};
use crate::game::checker::Order;
use std::collections::HashSet;

use super::checker::Checker;

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

    pub fn print_board(&self) {
        self.board.print_board();
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

        if match self.order {
            Order::WHITE if pos_to.1 == self.board.get_board_size().1 as i32 - 1 => true,
            Order::BLACK if pos_to.1 == 0 => true,
            _ => false,
        } {
            self.board.set_pos_king((pos_to.0 as u32, pos_to.1 as u32));
        }

        for i in 1..((pos_to.0 as i32 - pos_from.0 as i32).abs()) {
            let x_sign = (pos_to.0 - pos_from.0) / (pos_to.0 - pos_from.0).abs();
            let y_sign = (pos_to.1 - pos_from.1) / (pos_to.1 - pos_from.1).abs();

            if !self
                .board
                .is_pos_empty(((pos_from.0 + i * x_sign), (pos_from.1 + i * y_sign)))
            {
                self.board.remove_pos_checker((
                    (pos_from.0 + i * x_sign) as u32,
                    (pos_from.1 + i * y_sign) as u32,
                ));

                if !self
                    .board
                    .get_kill_steps((pos_to.0 as u32, pos_to.1 as u32))
                    .is_empty()
                {
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

    pub fn get_board_ref(&self) -> &Vec<Vec<Option<Checker>>> {
        self.board.get_board_ref()
    }

    pub fn get_board_size(&self) -> (usize, usize) {
        self.board.get_board_size()
    }

    pub fn get_board_order_pos(&self, pos: (i32, i32)) -> Option<Order> {
        match self.board.checker_order(pos) {
            CellKind::Some(v) => Some(v),
            _ => None,
        }
    }

    pub fn get_pos_is_king(&self, pos: (i32, i32)) -> bool {
        self.board.get_pos_is_king(pos)
    }

    pub fn input_vector(&self, order: Order, pos: &((u32, u32), (u32, u32))) -> Vec<f32> {
        //  0..31       - items with your color
        //  32..63     - items with oponents colors
        //  64..95    - king items with your color
        //  96..127    - king items with oponents color
        //  128..159    - movement start position
        //  160..191    - movement end position

        let board = self.get_board_ref();

        let input_board = |b: &mut Vec<Vec<f32>>, ord: Order, is_king: bool| {
            for x in 0..8 {
                for y in 0..8 {
                    b[x][y] = match &board[x][y] {
                        Some(v) if v.get_color() == ord && v.is_king() == is_king => 1.,
                        _ => 0.,
                    }
                }
            }
        };

        let mut wcol = vec![vec![0.; 8]; 8];
        let mut bcol = vec![vec![0.; 8]; 8];
        let mut wkcol = vec![vec![0.; 8]; 8];
        let mut bkcol = vec![vec![0.; 8]; 8];
        let mut stpos = vec![vec![0.; 8]; 8];
        let mut edpos = vec![vec![0.; 8]; 8];

        input_board(&mut wcol, Order::WHITE, false);
        input_board(&mut bcol, Order::BLACK, false);

        input_board(&mut wkcol, Order::WHITE, true);
        input_board(&mut bkcol, Order::BLACK, true);

        stpos[pos.0 .0 as usize][pos.0 .1 as usize] = 1.;
        edpos[pos.1 .0 as usize][pos.1 .1 as usize] = 1.;

        let mut inp: Vec<f32> = Vec::new();

        if let Order::WHITE = order {
            inp.extend(one_line(&wcol).iter());
            inp.extend(one_line(&bcol).iter());
            inp.extend(one_line(&wkcol).iter());
            inp.extend(one_line(&bkcol).iter());
            inp.extend(one_line(&stpos).iter());
            inp.extend(one_line(&edpos).iter());
        } else {
            transpose(&mut wcol);
            transpose(&mut bcol);
            transpose(&mut wkcol);
            transpose(&mut bkcol);
            transpose(&mut stpos);
            transpose(&mut edpos);

            inp.extend(one_line(&bcol).iter());
            inp.extend(one_line(&wcol).iter());
            inp.extend(one_line(&bkcol).iter());
            inp.extend(one_line(&wkcol).iter());
            inp.extend(one_line(&stpos).iter());
            inp.extend(one_line(&edpos).iter());
        }

        inp
    }
}

fn transpose(b: &mut Vec<Vec<f32>>) {
    b.reverse();
    for e in b {
        e.reverse();
    }
}

fn one_line(b: &Vec<Vec<f32>>) -> Vec<f32> {
    let mut v = Vec::new();

    for x in 0..8 {
        for y in 0..8 {
            if (x + y) % 2 == 0 {
                v.push(b[x][y]);
            }
        }
    }

    v
}
