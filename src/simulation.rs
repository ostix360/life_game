use crate::cell::Cell;
use crate::individual::predator::Predator;
use crate::individual::prey::Prey;
use kd_tree::KdTree;
use pyo3::{pyclass, pymethods};
use rand::Rng;
use std::sync::{Arc, Mutex};


pub fn get_cell(sim: &mut Simulation, x: i32, y: i32) -> Option<Arc<Mutex<Cell>>> {
    if x < 0 || x >= sim.width || y < 0 || y >= sim.height {
        return None;
    }
    let cell = Arc::clone(&sim.grid[x as usize][y as usize]);
    Some(cell)
}


#[pyclass]
pub struct Simulation {
    width: i32,
    height: i32,
    grid: Vec<Vec<Arc<Mutex<Cell>>>>,
    prey_position: Vec<[i32; 2]>,
    predator_position: Vec<(i32, i32)>,
    prey_reproduction_factor: f32,
    prey_moving_factor: f32,
    predator_reproduction_factor: f32,
    predator_moving_factor: f32,
    predator_hunting_factor: f32,
    predator_death_rate: f32,
    predator_max_hunger: u32,
    nb_initial_prey: u32,
    nb_initial_predators: u32,
}

#[pymethods]
impl Simulation {
    
    #[new]
    pub fn new(width: i32, height: i32, prey_reproduction_factor: f32, prey_moving_factor: f32, predator_reproduction_factor: f32, predator_moving_factor: f32, predator_hunting_factor: f32, predator_death_rate: f32, predator_max_hunger: u32, nb_initial_prey: u32, nb_initial_predators: u32) -> Self {
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
            predator_death_rate,
            predator_max_hunger,
            nb_initial_prey,
            nb_initial_predators,
        }
    }

    pub fn init_grid(&mut self){
        for i in 0..self.get_width() {
            let mut row = Vec::new();
            for j in 0..self.get_height() {
                row.push(Arc::new(Mutex::new(Cell::new(i, j))));
            }
            self.grid.push(row);
        }
    }


    fn get_width(&self) -> i32 {
        self.width
    }

    fn get_height(&self) -> i32 {
        self.height
    }

    pub fn init_simulation(&mut self) {
        let width = self.get_width();
        let height = self.get_height();
        for _ in 0..self.nb_initial_prey {
            let mut rng = rand::rng();
            let x = rng.random_range(0..width);
            let y = rng.random_range(0..height);
            get_cell(self, x, y).unwrap().lock().unwrap().content = Some(Box::new(Prey::new(self.prey_reproduction_factor, self.prey_moving_factor)));
            get_cell(self, x, y).unwrap().lock().unwrap().is_empty = false;
            get_cell(self, x, y).unwrap().lock().unwrap().is_prey = true;
        }
        for _ in 0..self.nb_initial_predators {
            let mut rng = rand::rng();
            let x = rng.random_range(0..width);
            let y = rng.random_range(0..height);
            get_cell(self, x, y).unwrap().lock().unwrap().content = Some(Box::new(Predator::new(x, y, self.predator_reproduction_factor, self.predator_moving_factor, self.predator_hunting_factor, self.predator_death_rate, self.predator_max_hunger, width, height)));
            get_cell(self, x, y).unwrap().lock().unwrap().is_empty = false;
            get_cell(self, x, y).unwrap().lock().unwrap().is_predator = true;
        }
        for i in 0..width {
            for j in 0..height {
                for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                    let ni = (i + dx + width) % height;
                    let nj = (j + dy + width) % height;
                    if ni != i || nj != j {
                        let cell = get_cell(self, i, j).unwrap();
                        let neighbour = get_cell(self, ni, nj).unwrap();
                        cell.lock().unwrap().add_neighbour(neighbour);
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

    pub fn get_nb_prey(&self) -> usize {
        self.prey_position.len()
    }
    pub fn get_nb_predators(&self) -> usize {
        self.predator_position.len()
    }
    
    fn update_parallel(&mut self, i: i32, j: i32) {
        let mut prey_cell = Vec::new();
        let mut predator_cell = Vec::new();
        let mut predator_coords = Vec::new();
        self.prey_position.clear();

        for x in (0..self.get_width()).step_by(1) {
            for y in (0..self.get_height()).step_by(1) {
                if let Some(cell) = get_cell(self, i + x, j + y) {
                    let cell_ref = Arc::clone(&cell);
                    if cell_ref.lock().unwrap().is_prey() {
                        let x = cell_ref.lock().unwrap().x;
                        let y = cell_ref.lock().unwrap().y;
                        self.prey_position.push([x, y]);
                        prey_cell.push(cell_ref);
                    } else if cell_ref.lock().unwrap().is_predator() {
                        let x = cell_ref.lock().unwrap().x;
                        let y = cell_ref.lock().unwrap().y;
                        predator_coords.push((x, y));
                        predator_cell.push(cell_ref);
                    }
                }else{
                    println!("Cell not found at ({}, {})", i + x, j + y);
                }
            }
        }
        self.predator_position.clone_from(&predator_coords);

        let nearest_prey = self.get_nearest_preys(predator_coords);
        for cell in prey_cell {
            cell.lock().unwrap().update(None);
        }
        for (i, cell) in predator_cell.iter().enumerate() {
            let nearest_prey = if i < nearest_prey.len() {
                Some(nearest_prey[i])
            } else {
                None
            };
            cell.lock().unwrap().update(nearest_prey);
        }
    }

    pub fn simulate(&mut self) -> (Vec<[i32; 2]>, Vec<(i32, i32)>) {
        // for i in [0, 1, 2] {
        //     for j in [0, 1, 2] {
        //         self.update_parallel(i, j);
        //     }
        // }
        self.update_parallel(0, 0);
        (self.prey_position.clone(), self.predator_position.clone())
    }
}