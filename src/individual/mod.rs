pub(crate) mod prey;
pub(crate) mod predator;

use crate::cell::Cell;
use std::sync::{Arc, Mutex};

pub(crate) trait Individual{
    fn update(&mut self, nearest_prey: Option<(i32, i32)>, local_contents: &mut [Arc<Mutex<Cell>>], local_empty_cells: &mut Vec<Arc<Mutex<Cell>>>) -> bool;
}
