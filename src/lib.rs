
// This is a Rust library that simulates a simple ecosystem with prey and predators.

mod individual;

use rand::prelude::IndexedMutRandom;
use rand::Rng;


trait Individual {
    fn update(&self, nearest_prey: Option<Prey>, local_contents: &Vec<Cell>, local_empty_cells: &Vec<Cell>);
}


struct Cell {
    x: u32,
    y: u32,
    pub content: Option<Box<dyn Individual>>,
    neighbours: Vec<Cell>,
    simulation: & 'static Simulation,
    is_empty: bool,
    is_predator: bool,
    is_prey: bool,
}

impl Cell {
    fn new(x: u32, y: u32, simulation: & 'static Simulation) -> Self {
        Cell {
            x,
            y,
            content: None,
            neighbours: Vec::new(),
            simulation,
            is_empty: true,
            is_predator: false,
            is_prey: false,
        }
    }

    fn add_neighbour(&mut self, neighbour: Cell) {
        self.neighbours.push(neighbour);
    }

    fn remove_content(&mut self) {
        self.content = None;
        self.is_empty = true;
    }

    fn is_empty(&self) -> bool {
        self.is_empty
    }

    fn is_prey(&self) -> bool {
        self.is_prey
    }

    fn is_predator(&self) -> bool {
        self.is_predator
    }
}


struct Prey {
    reproduction_factor: f32,
    moving_factor: f32,
}

impl Prey {
    fn new(reproduction_factor: f32, moving_factor: f32) -> Self {
        Prey {
            reproduction_factor,
            moving_factor,
        }
    }
    
    fn move_to(&self, local_empty_cells: &mut Vec<Cell>) -> bool {
        if local_empty_cells.is_empty() {
            return false
        }
        let mut rng = rand::rng();
        let mut empty_cell = local_empty_cells.choose_mut(&mut rng).unwrap();
        empty_cell.content = Some(Box::new(Prey::new(self.reproduction_factor, self.moving_factor)));
        empty_cell.is_empty = false;
        empty_cell.is_prey = true;
        true
    }

    fn reproduce(&self, local_contents: &Vec<Cell>, local_empty_cells: &mut Vec<Cell>) -> bool {
        if local_empty_cells.is_empty() {
            return false
        }
        let nb_prey = local_contents.iter().filter(|cell| cell.is_prey).count();
        if nb_prey == 0 || nb_prey >= 4 {
            return false
        }
        let mut rng = rand::rng();
        for cell in local_contents.iter() {
            let rng_nb: f32 = rng.random();
            if cell.is_prey && rng_nb < self.reproduction_factor {
                let mut empty_cell = local_empty_cells.choose_mut(&mut rng).unwrap();
                empty_cell.content = Some(Box::new(Prey::new(self.reproduction_factor, self.moving_factor)));
                empty_cell.is_empty = false;
                empty_cell.is_prey = true;
                return true
            }
        }
        false
    }
}

impl Individual for Prey {
    fn update(&self, nearest_prey: Option<Prey>, local_contents: &Vec<Cell>, local_empty_cells: &Vec<Cell>) {

    }
}

struct Predator {
    x: u32,
    y: u32,
    reproduction_factor: f32,
    moving_factor: f32,
    hunting_factor: f32,
    hunger: u32,
    death_rate: f32,
    max_hunger: u32,
}

struct Simulation {
    width: u32,
    height: u32,
    grid: Vec<Vec<Cell>>,
    prey_position: Vec<(u32, u32)>,
    predator_position: Vec<(u32, u32)>,
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
