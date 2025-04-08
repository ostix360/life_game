use std::cell::RefCell;
use std::rc::Rc;
use crate::individual::Individual;

pub struct Cell {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub content: Option<Box<dyn Individual + Send + Sync>>,
    neighbours: Vec<Rc<RefCell<Cell>>>,
    pub(crate) is_empty: bool,
    pub(crate) is_predator: bool,
    pub(crate) is_prey: bool,
}

impl Cell {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Cell {
            x,
            y,
            content: None,
            neighbours: Vec::with_capacity(8),
            is_empty: true,
            is_predator: false,
            is_prey: false,
        }
    }
    
    pub(crate) fn update(&mut self, nearest_prey: Option<(i32, i32)>) {
        if let Some(content) = &mut self.content {
            let mut local_empty_cells = self.neighbours
                .iter()
                .filter(|cell| cell.borrow_mut().is_empty())
                .map(Rc::clone)
                .collect::<Vec<_>>();
            
            let is_dead = content.update(nearest_prey, &mut self.neighbours, &mut local_empty_cells);
            if is_dead {
                self.empty();
            }
        }
    }
    pub(crate) fn add_neighbour(&mut self, neighbour: Rc<RefCell<Cell>>) {
        self.neighbours.push(neighbour);
    }

    pub(crate) fn empty(&mut self) {
        self.content = None;
        self.is_empty = true;
        self.is_predator = false;
        self.is_prey = false;
    }

    fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub(crate) fn is_prey(&self) -> bool {
        self.is_prey
    }

    pub(crate) fn is_predator(&self) -> bool {
        self.is_predator
    }
}