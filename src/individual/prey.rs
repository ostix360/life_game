use crate::cell::Cell;
use crate::individual::Individual;
use rand::prelude::IndexedMutRandom;
use rand::Rng;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct Prey {
    reproduction_factor: f32,
    moving_factor: f32,
}

impl Prey {
    pub(crate) fn new(reproduction_factor: f32, moving_factor: f32) -> Self {
        Prey {
            reproduction_factor,
            moving_factor,
        }
    }

    fn move_to(&self, local_empty_cells: &mut [Arc<Mutex<Cell>>]) -> bool {
        if local_empty_cells.is_empty() {
            return false
        }
        let mut rng = rand::rng();
        let empty_cell = local_empty_cells.choose_mut(&mut rng).unwrap();
        empty_cell.lock().unwrap().content = Some(Box::new(self.clone()));
        empty_cell.lock().unwrap().is_empty = false;
        empty_cell.lock().unwrap().is_prey = true;
        true
    }

    fn reproduce(&self, local_contents: &[Arc<Mutex<Cell>>], local_empty_cells: &mut [Arc<Mutex<Cell>>]) -> bool {
        if local_empty_cells.is_empty() {
            return false
        }
        let nb_prey = local_contents.iter().filter(|cell| cell.lock().unwrap().is_prey()).count();
        if nb_prey == 0 || nb_prey >= 4 {
            return false
        }
        let mut rng = rand::rng();
        for cell in local_contents {
            let rng_nb: f32 = rng.random();
            if cell.lock().unwrap().is_prey() && rng_nb < self.reproduction_factor {
                let empty_cell = local_empty_cells.choose_mut(&mut rng).unwrap();
                empty_cell.lock().unwrap().content = Some(Box::new(Prey::new(self.reproduction_factor, self.moving_factor)));
                empty_cell.lock().unwrap().is_empty = false;
                empty_cell.lock().unwrap().is_prey = true;
                return true
            }
        }
        false
    }
}

impl Individual for Prey {
    fn update(&mut self, _nearest_prey: Option<(i32, i32)>, local_contents: &mut [Arc<Mutex<Cell>>], local_empty_cells: &mut [Arc<Mutex<Cell>>]) -> bool {
        if self.reproduce(local_contents, local_empty_cells){
            return false
        }
        self.move_to(local_empty_cells)
    }
}