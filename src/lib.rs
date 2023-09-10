pub mod bot;
pub mod controller;
pub mod game;
pub mod gui;
mod py_wrapper;

use py_wrapper::{input_vector_by_board, Checkers, PyBoard, PyOrder};
use pyo3::prelude::*;

#[pymodule]
fn game_checkers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Checkers>()?;
    m.add_class::<PyOrder>()?;
    m.add_class::<PyBoard>()?;
    m.add_function(wrap_pyfunction!(input_vector_by_board, m)?)?;
    Ok(())
}
