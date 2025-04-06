use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::IndexedMutRandom;
use rand::Rng;

use crate::cell::Cell;
use crate::individual::Individual;

#[derive(Clone)]
pub(crate) struct Predator {
    x: i32,
    y: i32,
    reproduction_factor: f32,
    moving_factor: f32,
    hunting_factor: f32,
    hunger: u32,
    death_rate: f32,
    max_hunger: u32,
    sim_width: i32,
    sim_height: i32,
}

impl Predator {
    pub(crate) fn new(x: i32, y: i32, reproduction_factor: f32, moving_factor: f32, hunting_factor: f32, max_hunger: u32, sim_width: i32, sim_height: i32) -> Self {
        Predator {
            x,
            y,
            reproduction_factor,
            moving_factor,
            hunting_factor,
            hunger: 0,
            death_rate: 0.1,
            max_hunger,
            sim_width,
            sim_height,
        }
    }
    
    fn hunt(&mut self, local_contents: &mut Vec<Rc<RefCell<Cell>>>) -> bool {
        for cell in local_contents.iter_mut() {
            let rng_num: f32 = rand::rng().random();
            if cell.borrow().is_prey() &&  rng_num < self.hunting_factor {
                self.hunger = 0;
                cell.borrow_mut().empty();
                return true;
            }
        }
        return false;
    }
    
    fn reproduce<'s>(&mut self, local_contents: &Vec<Rc<RefCell<Cell>>>, local_empty_cells: &mut Vec<Rc<RefCell<Cell>>>) -> bool {
        let nbr_predators = local_contents.iter().filter(|cell| cell.borrow().is_predator()).count();
        let rng_num: f32 = rand::rng().random();
        if nbr_predators == 0 || nbr_predators > 3 {
            return false;
        }
        if rng_num < self.reproduction_factor {
            if let Some(cell) = local_empty_cells.choose_mut(&mut rand::rng()) {
                cell.borrow_mut().content = Some(Box::new(Predator::new(self.x, self.y, self.reproduction_factor, self.moving_factor, self.hunting_factor, self.max_hunger, self.sim_width, self.sim_height)));
                cell.borrow_mut().is_empty = false;
                return true;
            }
        }
        false
    }
    
    fn move_to<'s>(&self, nearest_prey_pos: Option<(i32, i32)>, local_empty_cells: &mut Vec<Rc<RefCell<Cell>>>) -> bool {
        if let Some((x, y)) = nearest_prey_pos {
            let dx: i32 = if x > self.x { 1 } else if x == self.x { 0 } else { 1 };
            let dy: i32 = if y > self.y { 1 } else if y == self.y { 0 } else { -1 };
            let new_x = (self.x + dx) % self.sim_width;
            let new_y = (self.y + dy) % self.sim_height;
            for cell in local_empty_cells.iter_mut() {
                if cell.borrow().x == new_x && cell.borrow().y == new_y {
                    cell.borrow_mut().content = Some(Box::new(self.clone()));
                    cell.borrow_mut().is_empty = false;
                    cell.borrow_mut().is_predator = true;
                    return true;
                }
            }
        } else {
            // Move randomly to an empty cell
            if let Some(cell) = local_empty_cells.choose_mut(&mut rand::rng()) {
                cell.borrow_mut().content = Some(Box::new(self.clone()));
                cell.borrow_mut().is_empty = false;
                cell.borrow_mut().is_predator = true;
                return true;
            }
        }
        false
    }
}

impl Individual for Predator {
    fn update<'s>(&mut self, nearest_prey: Option<(i32, i32)>, local_contents: &mut Vec<Rc<RefCell<Cell>>>, local_empty_cells: &mut Vec<Rc<RefCell<Cell>>>) -> bool {
        self.hunger += 1;
        let rng_num: f32 = rand::rng().random();
        if rng_num < self.death_rate || self.hunger > self.max_hunger {
            return true;
        }
        self.hunt(local_contents); 
        if self.hunger > self.max_hunger / 2{
            return false;
        }
        if !local_empty_cells.is_empty() {
            if self.reproduce(local_contents, local_empty_cells) {
                return false;
            }
            return self.move_to(nearest_prey, local_empty_cells);
        }
        return false;
    }
}