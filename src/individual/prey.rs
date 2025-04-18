use std::cell::RefCell;
use std::rc::Rc;
use crate::cell::Cell;
use crate::individual::Individual;
use rand::prelude::IndexedMutRandom;
use rand::Rng;

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

    fn move_to(&self, local_empty_cells: &mut [Rc<RefCell<Cell>>]) -> bool {
        if local_empty_cells.is_empty() {
            return false
        }
        let mut rng = rand::rng();
        let rng_nb: f32 = rng.random();
        if rng_nb > self.moving_factor {
            return false
        }
        let empty_cell = local_empty_cells.choose_mut(&mut rng).unwrap();
        empty_cell.borrow_mut().content = Some(Box::new(self.clone()));
        empty_cell.borrow_mut().is_empty = false;
        empty_cell.borrow_mut().is_prey = true;
        true
    }

    fn reproduce(&self, local_contents: &[Rc<RefCell<Cell>>], local_empty_cells: &mut [Rc<RefCell<Cell>>]) -> bool {
        if local_empty_cells.is_empty() {
            return false
        }
        let nb_prey = local_contents.iter().filter(|cell| cell.borrow().is_prey()).count();
        if nb_prey == 0 || nb_prey >= 4 {
            return false
        }
        let mut rng = rand::rng();
        for cell in local_contents {
            let rng_nb: f32 = rng.random();
            if cell.borrow().is_prey() && rng_nb < self.reproduction_factor {
                let empty_cell = local_empty_cells.choose_mut(&mut rng).unwrap();
                empty_cell.borrow_mut().content = Some(Box::new(Prey::new(self.reproduction_factor, self.moving_factor)));
                empty_cell.borrow_mut().is_empty = false;
                empty_cell.borrow_mut().is_prey = true;
                return true
            }
        }
        false
    }
}

impl Individual for Prey {
    fn update(&mut self, _nearest_prey: Option<(i32, i32)>, local_contents: &mut [Rc<RefCell<Cell>>], local_empty_cells: &mut Vec<Rc<RefCell<Cell>>>) -> bool {
        if self.reproduce(local_contents, local_empty_cells){
            return false
        }
        self.move_to(local_empty_cells)
    }
}