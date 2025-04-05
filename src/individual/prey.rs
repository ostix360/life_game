use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::*;
use rand::rng;

use crate::cell::Cell;
use crate::individual::Individual;

/// Prey represents herbivores in the simulation
/// They can move and reproduce but can be eaten by predators
#[derive(Clone)]
pub struct Prey {
    /// Chance of reproduction per update
    reproduction_rate: f32,
    /// Chance of moving per update
    moving_factor: f32,
    /// Current x position
    x: i32,
    /// Current y position
    y: i32,
}

impl Prey {
    /// Create a new prey at position (x, y)
    pub fn new(reproduction_rate: f32, moving_factor: f32, x: i32, y: i32) -> Self {
        Self {
            reproduction_rate,
            moving_factor,
            x,
            y,
        }
    }

    /// Move the prey to an empty cell if possible
    /// 
    /// # Returns
    /// * `bool` - Whether the prey moved
    fn move_to(&mut self, local_empty_cells: &Vec<Rc<RefCell<Cell>>>) -> bool {
        // If no empty cells, can't move
        if local_empty_cells.is_empty() {
            return false;
        }
        
        // Roll for movement chance
        let mut rng = rng();
        if rng.random::<f32>() >= self.moving_factor {
            return false;
        }
        
        // Choose a random empty cell
        if let Some(cell) = local_empty_cells.choose(&mut rng) {
            let mut cell_mut = cell.borrow_mut();
            
            // Update position
            self.x = cell_mut.x;
            self.y = cell_mut.y;
            
            // Move to new cell
            cell_mut.set_content(Box::new(self.clone()), true, false);
            
            return true;
        }
        
        false
    }

    /// Try to reproduce into an empty neighboring cell
    /// 
    /// # Returns
    /// * `bool` - Whether reproduction occurred
    fn reproduce(&self, local_contents: &Vec<Rc<RefCell<Cell>>>, local_empty_cells: &Vec<Rc<RefCell<Cell>>>) -> bool {
        
        let nb_prey = local_contents
            .iter()
            .filter(|cell| cell.borrow().is_prey)
            .count();
        
        // Reproduction rules: need at least one other prey but not overcrowded
        if nb_prey == 0 || nb_prey >= 4 {
            return false;
        }
        
        // Check if reproduction occurs (based on probability)
        let mut rng = rng();
        if rng.random::<f32>() >= self.reproduction_rate {
            return false;
        }
        
        // Choose a random empty cell for the offspring
        if let Some(cell) = local_empty_cells.choose(&mut rng) {
            let mut cell_mut = cell.borrow_mut();
            
            // Create new prey at the empty cell
            let new_prey = Prey::new(
                self.reproduction_rate,
                self.moving_factor,
                cell_mut.x,
                cell_mut.y
            );
            
            // Place the new prey
            cell_mut.set_content(Box::new(new_prey), true, false);
            
            return true;
        }
        
        false
    }
}

impl Individual for Prey {
    fn update(
        &mut self,
        _nearest_prey: Option<(i32, i32)>, // Prey doesn't need this information
        local_contents: Vec<Rc<RefCell<Cell>>>,
        local_empty_cells: Vec<Rc<RefCell<Cell>>>
    ) -> bool {
        // Try to reproduce first
        if self.reproduce(&local_contents, &local_empty_cells) {
            return false; // We stay in current cell
        }
        
        // If not reproducing, try to move
        self.move_to(&local_empty_cells)
    }
    
    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    fn is_prey(&self) -> bool {
        true
    }
    
    fn is_predator(&self) -> bool {
        false
    }
}