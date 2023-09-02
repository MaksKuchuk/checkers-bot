use std::collections::HashSet;

use crate::checker::{Checker, Order};

pub enum CellKind<T> {
    OutOfBoard,
    Empty,
    Some(T),
}

pub struct Board {
    board: Vec<Vec<Option<Checker>>>,
}

impl Board {
    pub fn new(brd_size: (usize, usize)) -> Board {
        let mut brd: Vec<Vec<Option<Checker>>> = vec![vec![None; brd_size.1]; brd_size.0];

        // for x in (0..brd_size.0).step_by(2) {
        //     brd[x][0] = Some(Checker::new(Order::WHITE));
        //     brd[x][2] = Some(Checker::new(Order::WHITE));
        // }
        // for x in (1..brd_size.0 as usize).step_by(2) {
        //     brd[x][1] = Some(Checker::new(Order::WHITE));
        // }

        // for x in (0..brd_size.0).step_by(2) {
        //     brd[x][brd_size.1 - 2] = Some(Checker::new(Order::BLACK));
        // }
        // for x in (1..brd_size.0).step_by(2) {
        //     brd[x][brd_size.1 - 1] = Some(Checker::new(Order::BLACK));
        //     brd[x][brd_size.1 - 3] = Some(Checker::new(Order::BLACK));
        // }

        brd[6][6] = Some(Checker::new(Order::WHITE));
        brd[3][3] = Some(Checker::new(Order::BLACK));
        brd[3][5] = Some(Checker::new(Order::BLACK));
        brd[3][7] = Some(Checker::new(Order::BLACK));

        Board { board: brd }
    }

    pub fn print_board(&self) {
        for y in (0..self.board[0].len()).rev() {
            for x in 0..self.board.len() {
                match &self.board[x][y] {
                    None => print!(" "),
                    Some(obj) => {
                        print!(
                            "{}",
                            if let Order::WHITE = obj.get_color() {
                                '*'
                            } else {
                                '-'
                            }
                        )
                    }
                }
            }
            println!(" ");
        }
    }

    fn get_general_steps(&self, pos: (u32, u32)) -> HashSet<(u32, u32)> {
        let (ord, is_king) = match &self.board[pos.0 as usize][pos.1 as usize] {
            None => return HashSet::new(),
            Some(c) => (c.get_color(), c.is_king()),
        };

        let mut steps: HashSet<(u32, u32)> = HashSet::new();
        let ways_top = [(-1, 1), (1, 1)];
        let ways_down = [(-1, -1), (1, -1)];

        let pos = (pos.0 as i32, pos.1 as i32);

        if is_king {
            let maxx = std::cmp::max(self.board.len() as i32, self.board[0].len() as i32);
            for xy in 1..maxx {
                if self.is_pos_empty((pos.0 + xy, pos.1 + xy)) {
                    steps.insert(((pos.0 + xy) as u32, (pos.1 + xy) as u32));
                } else {
                    break;
                }
            }
            for xy in 1..maxx {
                if self.is_pos_empty((pos.0 + xy, pos.1 - xy)) {
                    steps.insert(((pos.0 + xy) as u32, (pos.1 - xy) as u32));
                } else {
                    break;
                }
            }
            for xy in 1..maxx {
                if self.is_pos_empty((pos.0 - xy, pos.1 + xy)) {
                    steps.insert(((pos.0 - xy) as u32, (pos.1 + xy) as u32));
                } else {
                    break;
                }
            }
            for xy in 1..maxx {
                if self.is_pos_empty((pos.0 - xy, pos.1 - xy)) {
                    steps.insert(((pos.0 - xy) as u32, (pos.1 - xy) as u32));
                } else {
                    break;
                }
            }
        } else {
            for (x, y) in if ord == Order::WHITE {
                ways_top
            } else {
                ways_down
            } {
                match self.checker_order((pos.0 as i32 + x, pos.1 as i32 + y)) {
                    CellKind::Empty => {
                        steps.insert(((pos.0 as i32 + x) as u32, (pos.1 as i32 + y) as u32))
                    }
                    _ => false,
                };
            }
        }

        steps
    }

    pub fn get_kill_steps(&self, pos: (u32, u32)) -> HashSet<(u32, u32)> {
        let (ord, is_king) = match &self.board[pos.0 as usize][pos.1 as usize] {
            None => return HashSet::new(),
            Some(c) => (c.get_color(), c.is_king()),
        };

        let mut steps: HashSet<(u32, u32)> = HashSet::new();
        let ways_kill = [(-2, -2), (-2, 2), (2, -2), (2, 2)];

        let pos = (pos.0 as i32, pos.1 as i32);

        if is_king {
            let mut f = false;
            let maxx = std::cmp::max(self.board.len() as i32, self.board[0].len() as i32);
            for xy in 1..maxx {
                if self.is_pos_empty((pos.0 + xy, pos.1 + xy)) {
                    if f {
                        steps.insert(((pos.0 + xy) as u32, (pos.1 + xy) as u32));
                    }
                } else {
                    if f {
                        break;
                    }
                    f = true;
                }
            }
            f = false;
            for xy in 1..maxx {
                if self.is_pos_empty(((pos.0 + xy) as i32, pos.1 - xy)) {
                    if f {
                        steps.insert(((pos.0 + xy) as u32, (pos.1 - xy) as u32));
                    }
                } else {
                    if f {
                        break;
                    }
                    f = true;
                }
            }
            f = false;
            for xy in 1..maxx {
                if self.is_pos_empty((pos.0 - xy, pos.1 + xy)) {
                    if f {
                        steps.insert(((pos.0 - xy) as u32, (pos.1 + xy) as u32));
                    }
                } else {
                    if f {
                        break;
                    }
                    f = true;
                }
            }
            f = false;
            for xy in 1..maxx {
                if self.is_pos_empty((pos.0 - xy, pos.1 - xy)) {
                    if f {
                        steps.insert(((pos.0 - xy) as u32, (pos.1 - xy) as u32));
                    }
                } else {
                    if f {
                        break;
                    }
                    f = true;
                }
            }
        } else {
            for (x, y) in ways_kill {
                match self.checker_order((pos.0 as i32 + x, pos.1 as i32 + y)) {
                    CellKind::Empty => {
                        match self.checker_order((pos.0 as i32 + x / 2, pos.1 as i32 + y / 2)) {
                            CellKind::Some(v) if v != ord => {
                                steps.insert(((pos.0 as i32 + x) as u32, (pos.1 as i32 + y) as u32))
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                };
            }
        }

        steps
    }

    fn get_kill_steps_for_all(&self, ord: Order) -> HashSet<(u32, u32)> {
        let mut steps: HashSet<(u32, u32)> = HashSet::new();

        for x in 0..self.board.len() {
            for y in 0..self.board[0].len() {
                match self.checker_order((x as i32, y as i32)) {
                    CellKind::Some(c) if c == ord => (),
                    _ => continue,
                }
                steps.extend(self.get_kill_steps((x as u32, y as u32)))
            }
        }

        steps
    }

    pub fn get_possible_steps(
        &self,
        pos: (u32, u32),
        must_kill_checker: Option<(u32, u32)>,
    ) -> (HashSet<(u32, u32)>, HashSet<(u32, u32)>) {
        let ord = match self.checker_order((pos.0 as i32, pos.1 as i32)) {
            CellKind::Some(v) => v,
            _ => return (HashSet::new(), HashSet::new()),
        };

        match must_kill_checker {
            Some(v) if v == pos => (),
            None => (),
            _ => return (HashSet::new(), HashSet::new()),
        }

        let mut steps: HashSet<(u32, u32)> = HashSet::new();

        let all_kill_steps = self.get_kill_steps_for_all(ord);

        if all_kill_steps.is_empty() {
            steps.extend(self.get_general_steps(pos));
        }

        let steps_kill = self.get_kill_steps(pos);

        (steps, steps_kill)
    }

    pub fn is_pos_empty(&self, pos: (i32, i32)) -> bool {
        if pos.0 < 0
            || pos.1 < 0
            || pos.0 >= self.board.len() as i32
            || pos.1 >= self.board[0].len() as i32
        {
            return false;
        }

        match self.board[pos.0 as usize][pos.1 as usize] {
            Some(_) => false,
            None => true,
        }
    }

    pub fn checker_order(&self, pos: (i32, i32)) -> CellKind<Order> {
        if pos.0 < 0
            || pos.0 >= self.board.len() as i32
            || pos.1 < 0
            || pos.1 >= self.board[0].len() as i32
        {
            return CellKind::OutOfBoard;
        }

        match &self.board[pos.0 as usize][pos.1 as usize] {
            Some(v) => CellKind::Some(v.get_color()),
            None => CellKind::Empty,
        }
    }

    pub fn remove_pos_checker(&mut self, pos: (u32, u32)) {
        self.board[pos.0 as usize][pos.1 as usize] = None;
    }

    pub fn move_pos_checker(&mut self, pos_from: (u32, u32), pos_to: (u32, u32)) {
        self.board[pos_to.0 as usize][pos_to.1 as usize] =
            self.board[pos_from.0 as usize][pos_from.1 as usize].take();
    }

    pub fn get_board_size(&self) -> (usize, usize) {
        (self.board.len(), self.board[0].len())
    }

    pub fn set_pos_king(&mut self, pos: (u32, u32)) {
        if !self.is_pos_empty((pos.0 as i32, pos.1 as i32)) {
            self.board[pos.0 as usize][pos.1 as usize]
                .as_mut()
                .unwrap()
                .set_king();
        }
    }
}
