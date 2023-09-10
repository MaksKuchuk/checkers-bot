use std::collections::HashSet;

use pyo3::prelude::*;

use crate::game::{
    checker::{Checker, Order},
    game_object::GameObject,
};

#[pyclass]
pub struct Checkers {
    game: GameObject,
}

#[pyclass(name = "Order")]
pub enum PyOrder {
    WHITE,
    BLACK,
}

#[derive(Clone)]
#[pyclass(name = "Checker")]
pub struct PyChecker {
    checker: Checker,
}

#[pyclass(name = "Board")]
pub struct PyBoard {
    board: Vec<Vec<Option<Checker>>>,
}

impl PyBoard {
    pub fn get(&self) -> &Vec<Vec<Option<Checker>>> {
        &self.board
    }
}

#[pymethods]
impl PyChecker {
    pub fn get_color(&self) -> PyOrder {
        match self.checker.get_color() {
            Order::WHITE => PyOrder::WHITE,
            Order::BLACK => PyOrder::BLACK,
        }
    }

    pub fn is_king(&self) -> bool {
        self.checker.is_king()
    }
}

#[pymethods]
impl Checkers {
    #[new]
    fn new() -> Checkers {
        Checkers {
            game: GameObject::new((8, 8)),
        }
    }

    fn print_board(&self) {
        self.game.print_board();
    }

    fn get_order(&self) -> PyOrder {
        match self.game.get_order() {
            Order::WHITE => PyOrder::WHITE,
            Order::BLACK => PyOrder::BLACK,
        }
    }

    fn get_loc_possible_steps(&self) -> HashSet<((u32, u32), (u32, u32))> {
        self.game.get_loc_possible_steps().clone()
    }

    fn make_step(&mut self, pos_from: (u32, u32), pos_to: (u32, u32)) -> bool {
        self.game.make_step(pos_from, pos_to)
    }

    fn is_win(&self) -> Option<PyOrder> {
        match self.game.is_win() {
            None => None,
            Some(v) => match v {
                Order::WHITE => Some(PyOrder::WHITE),
                Order::BLACK => Some(PyOrder::BLACK),
            },
        }
    }

    pub fn get_board(&self) -> PyBoard {
        PyBoard {
            board: self.game.get_board_ref().clone(),
        }
    }

    fn get_board_size(&self) -> (usize, usize) {
        self.game.get_board_size()
    }

    fn input_vector(&self, order: &PyOrder, pos: ((u32, u32), (u32, u32))) -> Vec<f64> {
        //  0..31       - items with your color
        //  32..63     - items with oponents colors
        //  64..95    - king items with your color
        //  96..127    - king items with oponents color
        //  128..159    - movement start position
        //  160..191    - movement end position

        let board = self.game.get_board_ref();

        let input_board = |b: &mut Vec<Vec<f64>>, ord: Order, is_king: bool| {
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

        let mut inp: Vec<f64> = Vec::new();

        if let PyOrder::WHITE = order {
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

#[pyfunction]
pub fn input_vector_by_board(
    brd: &PyBoard,
    order: &PyOrder,
    pos: ((u32, u32), (u32, u32)),
) -> Vec<f64> {
    //  0..31       - items with your color
    //  32..63     - items with oponents colors
    //  64..95    - king items with your color
    //  96..127    - king items with oponents color
    //  128..159    - movement start position
    //  160..191    - movement end position

    let board = brd.get();

    let input_board = |b: &mut Vec<Vec<f64>>, ord: Order, is_king: bool| {
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

    let mut inp: Vec<f64> = Vec::new();

    if let PyOrder::WHITE = order {
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

fn transpose(b: &mut Vec<Vec<f64>>) {
    b.reverse();
    for e in b {
        e.reverse();
    }
}

fn one_line(b: &Vec<Vec<f64>>) -> Vec<f64> {
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
