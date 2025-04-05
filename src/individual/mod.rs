pub mod prey;
pub mod predator;

use std::cell::RefCell;
use std::rc::Rc;
use crate::cell::Cell;

/// Individual trait defines behavior for all entities in the simulation
/// Both prey and predator implement this trait
pub trait Individual {
    /// Update the individual based on its surroundings
    /// 
    /// # Arguments
    /// * `current_cell` - The cell where this individual is located
    /// * `nearest_prey` - Optional position of the nearest prey (for predators)
    /// * `local_contents` - List of neighboring cells
    /// * `local_empty_cells` - List of empty neighboring cells for potential movement
    /// 
    /// # Returns
    /// * `bool` - Whether the individual died or moved (and should be removed from current cell)
    fn update(
        &mut self, 
        nearest_prey: Option<(i32, i32)>, 
        local_contents: Vec<Rc<RefCell<Cell>>>, 
        local_empty_cells: Vec<Rc<RefCell<Cell>>>
    ) -> bool;
    
    /// Get the position of this individual (x, y)
    fn get_position(&self) -> (i32, i32);
    
    /// Check if this individual is a prey
    fn is_prey(&self) -> bool;
    
    /// Check if this individual is a predator
    fn is_predator(&self) -> bool;
}
