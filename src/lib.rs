// This is a Rust library that simulates a simple ecosystem with prey and predators.
// The simulation uses a grid of cells, each of which can contain either a prey or predator.
// The simulation follows these rules:
// 1. Prey can move randomly and reproduce when near other prey
// 2. Predators hunt prey, move toward prey, and reproduce when near other predators
// 3. Predators die if they don't eat for too long

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;
use rand::{rng, Rng};
use kd_tree::{KdTree, KdPoint};
use ordered_float::OrderedFloat;

pub mod individual;
pub mod cell;

use crate::cell::Cell;
use crate::individual::prey::Prey;
use crate::individual::predator::Predator;

// Define a struct to represent a point in 2D space for the KD-tree
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl KdPoint for Point {
    type Scalar = OrderedFloat<f32>;
    type Dim = typenum::U2;

    fn at(&self, k: usize) -> Self::Scalar {
        match k {
            0 => OrderedFloat(self.x as f32),
            1 => OrderedFloat(self.y as f32),
            _ => unreachable!(),
        }
    }
}

/// Parameters for the simulation
pub struct SimulationConfig {
    /// Width of the simulation grid
    pub width: usize,
    /// Height of the simulation grid
    pub height: usize,
    /// Chance of prey reproduction per update
    pub prey_reproduction_rate: f32,
    /// Chance of prey movement per update
    pub prey_moving_factor: f32,
    /// Chance of successful predator hunting per update
    pub predator_hunting_factor: f32,
    /// Chance of predator reproduction per update
    pub predator_reproduction_rate: f32,
    /// Number of updates without food before predator dies
    pub predator_death_after: u32,
    /// Initial number of prey in the grid
    pub nb_prey_init: usize,
    /// Initial number of predators in the grid
    pub nb_predator_init: usize,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            prey_reproduction_rate: 0.3,
            prey_moving_factor: 0.5,
            predator_hunting_factor: 0.5,
            predator_reproduction_rate: 0.4,
            predator_death_after: 25,
            nb_prey_init: 150,
            nb_predator_init: 50,
        }
    }
}

/// Main simulation structure
pub struct Simulation {
    /// Width of the simulation grid
    width: usize,
    /// Height of the simulation grid
    height: usize,
    /// 2D grid of cells
    grid: Vec<Vec<Rc<RefCell<Cell>>>>,
    /// Set of prey positions (x, y)
    prey_positions: HashSet<(i32, i32)>,
    /// Set of predator positions (x, y)
    predator_positions: HashSet<(i32, i32)>,
    /// KD-Tree for efficiently finding nearest prey
    prey_kdtree: Option<KdTree<Point>>,
    /// Count of prey in the simulation
    nb_prey: usize,
    /// Count of predators in the simulation
    nb_predator: usize,
    /// Neighbor offsets for calculating adjacent cells
    neighbor_offsets: Vec<(i32, i32)>,
    /// Configuration parameters
    config: SimulationConfig,
}

impl Simulation {
    /// Create a new simulation with the given configuration
    pub fn new(config: SimulationConfig) -> Self {
        // Pre-compute offsets for neighboring cells
        let neighbor_offsets = vec![
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];
        
        // Initialize the simulation
        let mut sim = Self {
            width: config.width,
            height: config.height,
            grid: vec![vec![]; config.height],
            prey_positions: HashSet::new(),
            predator_positions: HashSet::new(),
            prey_kdtree: None,
            nb_prey: 0,
            nb_predator: 0,
            neighbor_offsets,
            config,
        };
        
        // Initialize the grid with empty cells
        sim.init_grid();
        
        sim
    }
    
    /// Initialize the grid with empty cells and set up neighbors
    fn init_grid(&mut self) {
        // Create empty cells
        self.grid = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| Rc::new(RefCell::new(Cell::new(x as i32, y as i32))))
                    .collect()
            })
            .collect();
        
        // Set up neighborhood connections
        for y in 0..self.height {
            for x in 0..self.width {
                // Create a mutable reference to the current cell
                let current_cell = Rc::clone(&self.grid[y][x]);
                
                // Add all neighboring cells
                for &(dx, dy) in &self.neighbor_offsets {
                    // Calculate neighbor coordinates with wrapping
                    let nx = (x as i32 + dx + self.width as i32) % self.width as i32;
                    let ny = (y as i32 + dy + self.height as i32) % self.height as i32;
                    
                    // Get reference to the neighbor cell
                    let neighbor = Rc::clone(&self.grid[ny as usize][nx as usize]);
                    
                    // Add the neighbor to the current cell
                    current_cell.borrow_mut().add_neighbor(neighbor);
                }
            }
        }
        
        // Initialize with random prey
        let mut rng_ = rng();
        for _ in 0..self.config.nb_prey_init {
            let x = rng_.random_range(0..self.width);
            let y = rng_.random_range(0..self.height);
            
            let prey = Prey::new(
                self.config.prey_reproduction_rate,
                self.config.prey_moving_factor,
                x as i32,
                y as i32
            );
            
            // Add prey to the cell
            self.grid[y][x].borrow_mut().set_content(Box::new(prey), true, false);
            
            // Track prey position
            self.prey_positions.insert((x as i32, y as i32));
            self.nb_prey += 1;
        }
        
        // Initialize with random predators
        for _ in 0..self.config.nb_predator_init {
            let x = rng_.random_range(0..self.width);
            let y = rng_.random_range(0..self.height);
            
            let predator = Predator::new(
                self.config.predator_hunting_factor,
                self.config.predator_reproduction_rate,
                self.config.predator_death_after,
                x as i32,
                y as i32,
                self.width as i32,   // Pass grid width
                self.height as i32   // Pass grid height
            );
            
            // Add predator to the cell
            self.grid[y][x].borrow_mut().set_content(Box::new(predator), false, true);
            
            // Track predator position
            self.predator_positions.insert((x as i32, y as i32));
            self.nb_predator += 1;
        }
        
        // Build the initial KD-Tree for prey positions
        self.update_prey_kdtree();
    }
    
    /// Update the KD-Tree with current prey positions for efficient nearest-neighbor search
    fn update_prey_kdtree(&mut self) {
        // Convert HashSet to Vec of Point structs for KdTree
        let prey_points: Vec<Point> = self.prey_positions
            .iter()
            .map(|&(x, y)| Point { x, y })
            .collect();
        
        // Only build KdTree if we have prey
        if !prey_points.is_empty() {
            // Build a new KdTree with current prey positions
            self.prey_kdtree = Some(KdTree::build_by_ordered_float(prey_points));
        } else {
            self.prey_kdtree = None;
        }
    }
    
    /// Find the nearest prey for a predator
    fn get_nearest_prey(&self, predator_pos: (i32, i32)) -> Option<(i32, i32)> {
        // If we have a KdTree and prey, find the nearest
        if let Some(ref tree) = self.prey_kdtree {
            if !self.prey_positions.is_empty() {
                // Convert predator position to point format
                let query = Point { x: predator_pos.0, y: predator_pos.1 };
                
                // Find the nearest prey
                if let Some(nearest) = tree.nearest(&query) {
                    return Some((nearest.item.x, nearest.item.y));
                }
            }
        }
        
        None
    }
    
    /// Update the simulation by one step
    pub fn update(&mut self) {
        
        // Create vectors to track cells that need updating
        let mut prey_cells = Vec::new();
        let mut predator_cells = Vec::new();
        
        // Collect cells that need to be updated
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.grid[y][x];
                let cell_ref = cell.borrow();
                
                if !cell_ref.is_empty {
                    if cell_ref.is_prey {
                        prey_cells.push((x, y));
                        // Add to temp prey positions for current state
                        self.prey_positions.insert((x as i32, y as i32));                        
                    } else if cell_ref.is_predator {
                        self.predator_positions.insert((x as i32, y as i32));
                        predator_cells.push((x, y));
                    }
                }
            }
        }
        
        // Reset population counters to ensure accurate counts
        self.nb_prey = prey_cells.len();
        self.nb_predator = predator_cells.len();
        
        // Update predators first
        for (x, y) in predator_cells {
            let cell = &self.grid[y][x];
            let predator_pos = (x as i32, y as i32);
            
            // Find nearest prey for this predator
            let nearest_prey = self.get_nearest_prey(predator_pos);
            
            // Update the predator
            {
                let mut cell_mut = cell.borrow_mut();
                let was_updated = cell_mut.update(nearest_prey);
                
                // If the predator moved or died, update our tracking
                if was_updated {
                    // Predator was removed or moved
                    self.predator_positions.remove(&(x as i32, y as i32));
                }
            }
        }
        
        // Update prey second (might have been eaten by predators already)
        for (x, y) in prey_cells {
            let cell = &self.grid[y][x];
            
            // Only update if the cell still contains prey (might have been eaten)
            if cell.borrow().is_prey {
                let mut cell_mut = cell.borrow_mut();
                let was_updated = cell_mut.update(None);
                
                // If the prey moved, update our tracking
                if was_updated {
                    // Prey was removed or moved
                    self.prey_positions.remove(&(x as i32, y as i32));
                }
            }
        }
        // Update the KD-Tree with new prey positions
        self.update_prey_kdtree();
        
    }
    
    
    /// Get the current number of prey in the simulation
    pub fn get_prey_count(&self) -> usize {
        self.nb_prey
    }
    
    /// Get the current number of predators in the simulation
    pub fn get_predator_count(&self) -> usize {
        self.nb_predator
    }
    
    /// Get positions of all prey
    pub fn get_prey_positions(&self) -> &HashSet<(i32, i32)> {
        &self.prey_positions
    }
    
    /// Get positions of all predators
    pub fn get_predator_positions(&self) -> &HashSet<(i32, i32)> {
        &self.predator_positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_initialization() {
        let config = SimulationConfig {
            width: 10,
            height: 10,
            nb_prey_init: 5,
            nb_predator_init: 3,
            ..Default::default()
        };
        
        let sim = Simulation::new(config);
        
        assert_eq!(sim.get_prey_count(), 5);
        assert_eq!(sim.get_predator_count(), 3);
    }
}
