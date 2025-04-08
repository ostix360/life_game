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
    pub(crate) fn new(x: i32, y: i32, reproduction_factor: f32, moving_factor: f32, hunting_factor: f32, death_rate: f32, max_hunger: u32, sim_width: i32, sim_height: i32) -> Self {
        Predator {
            x,
            y,
            reproduction_factor,
            moving_factor,
            hunting_factor,
            hunger: max_hunger/2,
            death_rate,
            max_hunger,
            sim_width,
            sim_height,
        }
    }

    
    fn hunt(&mut self, local_contents: &mut [Rc<RefCell<Cell>>], local_empty_cell: &mut Vec<Rc<RefCell<Cell>>>) -> bool {
        for cell in local_contents.iter_mut() {
            let rng_num: f32 = rand::rng().random();
            if cell.borrow().is_prey() &&  rng_num < self.hunting_factor {
                self.hunger = 0;
                cell.borrow_mut().empty();
                local_empty_cell.push(Rc::clone(cell));
                return true;
            }
        }
        false
    }
    
    fn reproduce(&mut self, local_contents: &[Rc<RefCell<Cell>>], local_empty_cells: &mut [Rc<RefCell<Cell>>]) -> bool {
        let nbr_predators = local_contents.iter().filter(|cell| cell.borrow().is_predator()).count();
        let rng_num: f32 = rand::rng().random();
        if nbr_predators == 0 || nbr_predators >= 4 {
            return false;
        }
        else if rng_num < self.reproduction_factor {
            if let Some(cell) = local_empty_cells.choose_mut(&mut rand::rng()) {
                let new_x = cell.borrow().x;
                let new_y = cell.borrow().y;
                cell.borrow_mut().content = Some(Box::new(Predator::new(new_x, new_y, self.reproduction_factor, self.moving_factor, self.hunting_factor, self.death_rate, self.max_hunger, self.sim_width, self.sim_height)));
                cell.borrow_mut().is_empty = false;
                cell.borrow_mut().is_predator = true;
                return true;
            }
        }
        false
    }

    fn move_to(&self, nearest_prey_pos: Option<(i32, i32)>, local_empty_cells: &mut [Rc<RefCell<Cell>>]) -> bool {
        if let Some((x, y)) = nearest_prey_pos {
            let dx: i32 = match x.cmp(&self.x) {
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => -1,
            };
            let dy: i32 = match y.cmp(&self.y) {
                std::cmp::Ordering::Greater => 1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Less => -1,
            };
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
        } else if let Some(cell) = local_empty_cells.choose_mut(&mut rand::rng()) {
            cell.borrow_mut().content = Some(Box::new(self.clone()));
            cell.borrow_mut().is_empty = false;
            cell.borrow_mut().is_predator = true;
            return true;
        }
        false
    }
}

impl Individual for Predator {
    fn update(&mut self, nearest_prey: Option<(i32, i32)>, local_contents: &mut [Rc<RefCell<Cell>>], local_empty_cells: &mut Vec<Rc<RefCell<Cell>>>) -> bool {
        self.hunger += 1;
        let rng_num: f32 = rand::rng().random();
        if rng_num < self.death_rate || self.hunger >= self.max_hunger {
            return true;
        }
        self.hunt(local_contents, local_empty_cells); 
        if self.hunger >= self.max_hunger / 2 || local_empty_cells.is_empty(){
            return false;
        }
        if self.reproduce(local_contents, local_empty_cells) {
            return false;
        }
        self.move_to(nearest_prey, local_empty_cells)
    }
}