use rand::prelude::IndexedMutRandom;
use rand::Rng;
use std::sync::{Arc, Mutex};

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
    
    fn hunt(&mut self, local_contents: &mut [Arc<Mutex<Cell>>]) -> bool {
        for cell in local_contents.iter_mut() {
            let rng_num: f32 = rand::rng().random();
            if cell.lock().unwrap().is_prey() &&  rng_num < self.hunting_factor {
                self.hunger = 0;
                cell.lock().unwrap().empty();
                return true;
            }
        }
        false
    }
    
    fn reproduce(&mut self, local_contents: &[Arc<Mutex<Cell>>], local_empty_cells: &mut [Arc<Mutex<Cell>>]) -> bool {
        let nbr_predators = local_contents.iter().filter(|cell| cell.lock().unwrap().is_predator()).count();
        let rng_num: f32 = rand::rng().random();
        if nbr_predators == 0 || nbr_predators > 3 {
            return false;
        }
        if rng_num < self.reproduction_factor {
            if let Some(cell) = local_empty_cells.choose_mut(&mut rand::rng()) {
                cell.lock().unwrap().content = Some(Box::new(Predator::new(self.x, self.y, self.reproduction_factor, self.moving_factor, self.hunting_factor, self.max_hunger, self.sim_width, self.sim_height)));
                cell.lock().unwrap().is_empty = false;
                return true;
            }
        }
        false
    }
    
    fn move_to(&self, nearest_prey_pos: Option<(i32, i32)>, local_empty_cells: &mut [Arc<Mutex<Cell>>]) -> bool {
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
                if cell.lock().unwrap().x == new_x && cell.lock().unwrap().y == new_y {
                    cell.lock().unwrap().content = Some(Box::new(self.clone()));
                    cell.lock().unwrap().is_empty = false;
                    cell.lock().unwrap().is_predator = true;
                    return true;
                }
            }
        } else {
            // Move randomly to an empty cell
            if let Some(cell) = local_empty_cells.choose_mut(&mut rand::rng()) {
                cell.lock().unwrap().content = Some(Box::new(self.clone()));
                cell.lock().unwrap().is_empty = false;
                cell.lock().unwrap().is_predator = true;
                return true;
            }
        }
        false
    }
}

impl Individual for Predator {
    fn update(&mut self, nearest_prey: Option<(i32, i32)>, local_contents: &mut [Arc<Mutex<Cell>>], local_empty_cells: &mut [Arc<Mutex<Cell>>]) -> bool {
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
        false
    }
}