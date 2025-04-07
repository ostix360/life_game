#[cfg(test)]
mod tests {
    use life_game::simulation::{get_cell, Simulation};

    fn create_test_simulation() -> Simulation {
        let mut sim = Simulation::new(
            10,
            10,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            0.1,
            20, 
            50,
            50
        );
        sim.init_grid();
        sim
    }

    #[test]
    fn get_cell_within_bounds() {
        let mut sim = create_test_simulation();
        let cell = get_cell(&mut sim, 5, 5);
        assert!(cell.is_some());
    }

    #[test]
    fn get_cell_out_of_bounds_negative() {
        let mut sim = create_test_simulation();
        let cell = get_cell(&mut sim, -1, -1);
        assert!(cell.is_none());
    }

    #[test]
    fn get_cell_out_of_bounds_positive() {
        let mut sim = create_test_simulation();
        let cell = get_cell(&mut sim, 10, 10);
        assert!(cell.is_none());
    }
    
}
