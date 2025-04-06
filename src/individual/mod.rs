pub(crate) mod prey;
pub(crate) mod predator;

use std::cell::RefCell;
use std::rc::Rc;
use prey::Prey;
use crate::cell::Cell;

pub(crate) trait Individual{
    fn update<'s>(&mut self, nearest_prey: Option<(i32, i32)>, local_contents: &mut Vec<Rc<RefCell<Cell>>>, local_empty_cells: &mut Vec<Rc<RefCell<Cell>>>) -> bool;
}
