use std::rc::Rc;
use std::cell::RefCell;
use crate::individual::Individual;

/// Cell struct represents a single cell in the simulation grid
/// Each cell can be empty or contain a prey or predator
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub content: Option<Box<dyn Individual>>,
    pub neighbors: Vec<Rc<RefCell<Cell>>>, // Using Rc<RefCell<>> for shared mutability
    pub is_empty: bool,
    pub is_prey: bool,
    pub is_predator: bool,
}

impl Cell {
    /// Create a new empty cell at position (x, y)
    pub fn new(x: i32, y: i32) -> Self {
        // Initially all cells are empty
        Self {
            x,
            y,
            content: None,
            neighbors: Vec::new(),
            is_empty: true,
            is_prey: false,
            is_predator: false,
        }
    }
    
    /// Add a neighboring cell reference
    pub fn add_neighbor(&mut self, neighbor: Rc<RefCell<Cell>>) {
        self.neighbors.push(neighbor);
    }

    /// Update the cell based on its current content
    pub fn update(&mut self, nearest_prey: Option<(i32, i32)>) -> bool {
        // We need to avoid the multiple mutable borrow issue
        // First, check if we have content to update
        if self.content.is_none() {
            return false;
        }
        
        // Create copies of the cell references to avoid multiple mutable borrows
        let local_contents: Vec<Rc<RefCell<Cell>>> = self.neighbors.clone();
        
        // Get neighbor cells that are empty for potential movement
        let local_empty_cells: Vec<Rc<RefCell<Cell>>> = local_contents
            .iter()
            .filter(|cell| cell.borrow().is_empty)
            .cloned()
            .collect();
        
        // Take ownership of the content temporarily
        let mut content = self.content.take().unwrap();
        
        // Update the individual
        let result = content.update(self, nearest_prey, local_contents, local_empty_cells);
        
        // If the individual didn't die or move, put it back
        if !result {
            self.content = Some(content);
        }
        
        // Return whether the individual died or moved
        result
    }

    /// Empty this cell (remove any content)
    pub fn empty(&mut self) {
        self.content = None;
        self.is_empty = true;
        self.is_predator = false;
        self.is_prey = false;
    }
    
    /// Set the content of this cell
    pub fn set_content(&mut self, content: Box<dyn Individual>, is_prey: bool, is_predator: bool) {
        self.content = Some(content);
        self.is_empty = false;
        self.is_prey = is_prey;
        self.is_predator = is_predator;
    }
}