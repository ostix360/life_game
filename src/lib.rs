
// This is a Rust library that simulates a simple ecosystem with prey and predators.

mod individual;
mod cell;

use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
use kd_tree;

use crate::cell::Cell;
use crate::individual::prey::Prey;
use crate::individual::predator::Predator;


struct Simulation {
    width: i32,
    height: i32,
    grid: Vec<Vec<Rc<RefCell<Cell<'static>>>>>,
    prey_position: Vec<(i32, i32)>,
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
    
    fn init_grid(mut self){
        let rc_self = Rc::new(RefCell::new(self));
        for i in 0..rc_self.borrow().get_width() {
            let mut row = Vec::new();
            for j in 0..rc_self.borrow().get_height() {
                row.push(Rc::new(RefCell::new(Cell::new(i, j, Rc::clone(&rc_self)))));
            }
            rc_self.borrow_mut().grid.push(row);
        }
    }
    
    fn get_cell(&mut self, x: i32, y: i32) -> Option<&mut Rc<RefCell<Cell<'static>>>> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(&mut self.grid[x as usize][y as usize])
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
