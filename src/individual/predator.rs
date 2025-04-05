use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::*;

use crate::cell::Cell;
use crate::individual::Individual;

/// Predator represents carnivores in the simulation
/// They hunt prey, move toward prey, and reproduce
#[derive(Clone)]
pub struct Predator {
    /// Chance to successfully hunt prey
    hunting_factor: f32,
    /// Chance of reproduction per update
    reproduction_rate: f32,
    /// Number of updates without food before death
    death_after: u32,
    /// Current hunger level (increases each update)
    hunger: u32,
    /// Chance of random death (natural causes)
    death_rate: f32,
    /// Current x position
    x: i32,
    /// Current y position
    y: i32,
    /// Width of the simulation grid (for wrapping)
    grid_width: i32,
    /// Height of the simulation grid (for wrapping)
    grid_height: i32,
}

impl Predator {
    /// Create a new predator at position (x, y)
    pub fn new(hunting_factor: f32, reproduction_rate: f32, death_after: u32, x: i32, y: i32, grid_width: i32, grid_height: i32) -> Self {
        Self {
            hunting_factor,
            reproduction_rate,
            death_after,
            // Start with hunger at half of death threshold
            hunger: death_after / 2,
            death_rate: 0.1,
            x,
            y,
            grid_width,
            grid_height,
        }
    }

    /// Try to hunt prey in neighboring cells
    /// 
    /// # Returns
    /// * `bool` - Whether hunting was successful
    fn hunt(&mut self, local_contents: &Vec<Rc<RefCell<Cell>>>) -> bool {
        // Look for prey in neighboring cells
        for cell in local_contents {
            let mut cell_ref = cell.borrow_mut();
            
            // If the cell contains prey, try to hunt it
            if cell_ref.is_prey {
                // Roll for hunting success
                let mut rng = thread_rng();
                if rng.gen::<f32>() < self.hunting_factor {
                    // Successful hunt: reset hunger and empty the prey cell
                    self.hunger = 0;
                    cell_ref.empty();
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Try to reproduce into an empty neighboring cell
    /// 
    /// # Returns
    /// * `bool` - Whether reproduction occurred
    fn reproduce(&self, local_contents: &Vec<Rc<RefCell<Cell>>>, local_empty_cells: &Vec<Rc<RefCell<Cell>>>) -> bool {
        // Can't reproduce if too hungry
        if self.hunger >= self.death_after / 2 {
            return false;
        }
        
        // Count neighboring predators
        let mut nb_predator = 0;
        for cell in local_contents {
            if cell.borrow().is_predator {
                nb_predator += 1;
            }
        }
        
        // Reproduction rules: need at least one other predator but not overcrowded
        if nb_predator == 0 || nb_predator >= 4 {
            return false;
        }
        
        // Check if reproduction occurs (based on probability)
        let mut rng = thread_rng();
        if rng.gen::<f32>() >= self.reproduction_rate {
            return false;
        }
        
        // Choose a random empty cell for the offspring
        if let Some(cell) = local_empty_cells.choose(&mut rng) {
            let mut cell_mut = cell.borrow_mut();
            
            // Create new predator at the empty cell
            let new_predator = Predator::new(
                self.hunting_factor,
                self.reproduction_rate,
                self.death_after,
                cell_mut.x,
                cell_mut.y,
                self.grid_width,
                self.grid_height
            );
            
            // Place the new predator
            cell_mut.set_content(Box::new(new_predator), false, true);
            
            return true;
        }
        
        false
    }
    
    /// Move toward the nearest prey if possible
    /// 
    /// # Returns
    /// * `bool` - Whether the predator moved
    fn move_to(&mut self, nearest_prey: Option<(i32, i32)>, local_empty_cells: &Vec<Rc<RefCell<Cell>>>) -> bool {
        // Can't move without empty cells
        if local_empty_cells.is_empty() {
            return false;
        }
        
        // If we know where a prey is, move toward it
        if let Some((prey_x, prey_y)) = nearest_prey {
            // Calculate direction to move (via shortest path)
            let mut dx = 0;
            let mut dy = 0;
            
            // Calculate x direction, handling wrapping at grid edges
            if prey_x != self.x {
                let direct_dist = (prey_x - self.x).abs();
                let wrap_dist = self.grid_width - direct_dist;
                
                // Determine if it's shorter to go directly or wrap around the edge
                if direct_dist <= wrap_dist {
                    // Direct path is shorter
                    dx = if prey_x > self.x { 1 } else { -1 };
                } else {
                    // Wrapping around is shorter
                    dx = if prey_x > self.x { -1 } else { 1 };
                }
            }
            
            // Calculate y direction, handling wrapping at grid edges
            if prey_y != self.y {
                let direct_dist = (prey_y - self.y).abs();
                let wrap_dist = self.grid_height - direct_dist;
                
                // Determine if it's shorter to go directly or wrap around the edge
                if direct_dist <= wrap_dist {
                    // Direct path is shorter
                    dy = if prey_y > self.y { 1 } else { -1 };
                } else {
                    // Wrapping around is shorter
                    dy = if prey_y > self.y { -1 } else { 1 };
                }
            }
            
            // Calculate new position (with wrapping)
            let new_x = (self.x + dx + self.grid_width) % self.grid_width;
            let new_y = (self.y + dy + self.grid_height) % self.grid_height;
            
            // Find an empty cell at the target position
            for cell in local_empty_cells {
                let cell_ref = cell.borrow();
                if cell_ref.x == new_x && cell_ref.y == new_y {
                    // Found the cell we want to move to
                    drop(cell_ref);  // Release the borrow
                    
                    // Move to the target cell
                    let mut cell_mut = cell.borrow_mut();
                    
                    // Update position
                    self.x = new_x;
                    self.y = new_y;
                    
                    // Move to new cell
                    cell_mut.set_content(Box::new(self.clone()), false, true);
                    
                    return true;
                }
            }
        }
        
        // If we don't know where prey is, or can't move toward it,
        // move randomly like prey does
        let mut rng = thread_rng();
        if let Some(cell) = local_empty_cells.choose(&mut rng) {
            let mut cell_mut = cell.borrow_mut();
            
            // Update position
            self.x = cell_mut.x;
            self.y = cell_mut.y;
            
            // Move to new cell
            cell_mut.set_content(Box::new(self.clone()), false, true);
            
            return true;
        }
        
        false
    }
}

impl Individual for Predator {
    fn update(
        &mut self,
        current_cell: &mut Cell,
        nearest_prey: Option<(i32, i32)>,
        local_contents: Vec<Rc<RefCell<Cell>>>,
        local_empty_cells: Vec<Rc<RefCell<Cell>>>
    ) -> bool {
        // Increase hunger each update
        self.hunger += 1;
        
        // Check for random death
        let mut rng = thread_rng();
        if rng.gen::<f32>() < self.death_rate {
            return true; // Die
        }
        
        // Check for death from starvation
        if self.hunger >= self.death_after {
            return true; // Die
        }
        
        // Try to hunt prey
        self.hunt(&local_contents);
        
        // Try to reproduce
        if self.reproduce(&local_contents, &local_empty_cells) {
            return false; // Stay in current cell
        }
        
        // Move toward prey or randomly
        self.move_to(nearest_prey, &local_empty_cells)
    }
    
    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    fn is_prey(&self) -> bool {
        false
    }
    
    fn is_predator(&self) -> bool {
        true
    }
}