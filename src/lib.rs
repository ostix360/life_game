#![warn(clippy::all)]
// This is a Rust library that simulates a simple ecosystem with prey and predators.

mod individual;
pub mod cell;
pub mod simulation;

use crate::simulation::Simulation;
use pyo3::prelude::*;

#[pymodule(name = "life_game")]
fn pp_sim(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<Simulation>()?;
    Ok(())
}

