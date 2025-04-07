use life_game::simulation::Simulation;

fn main() {
    let mut simulation = Simulation::new(
        100,
        100,
        0.5,
        0.5,
        0.5,
        0.5,
        0.5,
        0.1,
        25,
        1500,
        1000
    );
    simulation.init_grid();
    simulation.init_simulation();
    for i in 0..10000 {
        if i % 25 == 0 {
            simulation.simulate();
            println!("Step {}", i);
            println!("Nb of prey: {}", simulation.get_nb_prey());
            println!("Nb of predators: {}", simulation.get_nb_predators());
        }
    }
    
}