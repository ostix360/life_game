
// This is a Rust library that simulates a simple ecosystem with prey and predators.

mod individual;
mod cell;

use kd_tree;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use kd_tree::KdTree;
use crate::cell::Cell;
use crate::individual::predator::Predator;
use crate::individual::prey::Prey;


struct Simulation {
    width: i32,
    height: i32,
    grid: Vec<Vec<Rc<RefCell<Cell>>>>,
    prey_position: Vec<[i32; 2]>,
    predator_position: Vec<(i32, i32)>,
    prey_reproduction_factor: f32,
    prey_moving_factor: f32,
    predator_reproduction_factor: f32,
    predator_moving_factor: f32,
    predator_hunting_factor: f32,
    predator_hunger: u32,
    predator_death_rate: f32,
    predator_max_hunger: u32,
    nb_initial_prey: u32,
    nb_initial_predators: u32,
}

impl Simulation {
    fn new(width: i32, height: i32, prey_reproduction_factor: f32, prey_moving_factor: f32, predator_reproduction_factor: f32, predator_moving_factor: f32, predator_hunting_factor: f32, predator_hunger: u32, predator_death_rate: f32, predator_max_hunger: u32, nb_initial_prey: u32, nb_initial_predators: u32) -> Self {
        Simulation {
            width,
            height,
            grid: Vec::new(),
            prey_position: Vec::new(),
            predator_position: Vec::new(),
            prey_reproduction_factor,
            prey_moving_factor,
            predator_reproduction_factor,
            predator_moving_factor,
            predator_hunting_factor,
            predator_hunger,
            predator_death_rate,
            predator_max_hunger,
            nb_initial_prey,
            nb_initial_predators,
        }
    }
    
    fn init_grid(&mut self){
        for i in 0..self.get_width() {
            let mut row = Vec::new();
            for j in 0..self.get_height() {
                row.push(Rc::new(RefCell::new(Cell::new(i, j))));
            }
            self.grid.push(row);
        }
    }
    
    fn get_cell(&mut self, x: i32, y: i32) -> Option<Rc<RefCell<Cell>>> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        let cell = Rc::clone(&self.grid[x as usize][y as usize]);
        Some(cell)
    }
    
    fn get_width(&self) -> i32 {
        self.width
    }
    
    fn get_height(&self) -> i32 {
        self.height
    }
    
    fn init_simulation(&'static mut self) {
        let width = self.get_width();
        let height = self.get_height();
        for _ in 0..self.nb_initial_prey {
            let mut rng = rand::rng();
            let x = rng.random_range(0..width);
            let y = rng.random_range(0..height);
            self.get_cell(x, y).unwrap().borrow_mut().content = Some(Box::new(Prey::new(self.prey_reproduction_factor, self.prey_moving_factor)));
            self.get_cell(x, y).unwrap().borrow_mut().is_empty = false;
            self.get_cell(x, y).unwrap().borrow_mut().is_prey = true;
        }
        for _ in 0..self.nb_initial_predators {
            let mut rng = rand::rng();
            let x = rng.random_range(0..width);
            let y = rng.random_range(0..height);
            self.get_cell(x, y).unwrap().borrow_mut().content = Some(Box::new(Predator::new(x, y, self.predator_reproduction_factor, self.predator_moving_factor, self.predator_hunting_factor, self.predator_max_hunger, width, height)));
            self.get_cell(x, y).unwrap().borrow_mut().is_empty = false;
            self.get_cell(x, y).unwrap().borrow_mut().is_predator = true;
        }
        for i in 0..width {
            for j in 0..height {
                for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)] {
                    let ni = (i + dx + width) % height;
                    let nj = (j + dy + width) % height;
                    if ni != i || nj != j {
                        let cell = self.get_cell(i, j).unwrap();
                        let neighbour = self.get_cell(ni, nj).unwrap();
                        cell.borrow_mut().add_neighbour(neighbour);
                    }
                }
            }
        }
    }
    
    fn get_nearest_preys(&self, predator_pos: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let kd_tree = KdTree::build(self.prey_position.clone());
        let mut nearest_preys = Vec::new();
        for (x, y) in predator_pos {
            let nearest = kd_tree.nearest(&[x,y]);
            if let Some(nearest) = nearest {
                nearest_preys.push((nearest.item[0], nearest.item[1]));
            }
        }
        nearest_preys 
    }
    
    fn update_parallel(&mut self, i: i32, j: i32) {
        let mut prey_cell = Vec::new();
        let mut predator_cell = Vec::new();
        let mut predator_coords = Vec::new();
        self.prey_position.clear();
        
        for x in (0..self.get_width()).step_by(3) {
            for y in (0..self.get_height()).step_by(3) {
                let cell = self.get_cell(i + x, j + y).unwrap();
                let cell_ref = Rc::clone(&cell);
                if cell_ref.borrow().is_prey() {
                    self.prey_position.push([cell_ref.borrow().x, cell_ref.borrow().y]);
                    prey_cell.push(cell_ref);
                } else if cell_ref.borrow().is_predator() {
                    predator_coords.push((cell_ref.borrow().x, cell_ref.borrow().y));
                    predator_cell.push(cell_ref);
                }
            }
        }
        self.predator_position = predator_coords.clone();
        
        let nearest_prey = self.get_nearest_preys(predator_coords);
        for cell in prey_cell {
            cell.borrow_mut().update(None);
        }
        for (i, cell) in predator_cell.iter().enumerate() {
            let nearest_prey = if i < nearest_prey.len() {
                Some(nearest_prey[i])
            } else {
                None
            };
            cell.borrow_mut().update(nearest_prey);
        }
    }
    
    pub fn simulate(&mut self) {
        for i in [0, 1, 2] {
            for j in [0, 1, 2] {
                self.update_parallel(i, j);
            }
        }
    }
}
