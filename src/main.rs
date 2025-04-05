use std::time::Instant;
use life_game::{Simulation, SimulationConfig};

fn main() {
    // Create a simulation configuration with default parameters
    let config = SimulationConfig::default();
    
    // Print simulation parameters
    println!("Starting simulation with:");
    println!("  Grid size: {}x{}", config.width, config.height);
    println!("  Initial prey: {}", config.nb_prey_init);
    println!("  Initial predators: {}", config.nb_predator_init);
    println!("  Prey reproduction rate: {}", config.prey_reproduction_rate);
    println!("  Prey moving factor: {}", config.prey_moving_factor);
    println!("  Predator hunting factor: {}", config.predator_hunting_factor);
    println!("  Predator reproduction rate: {}", config.predator_reproduction_rate);
    println!("  Predator death after: {} steps without food", config.predator_death_after);
    
    // Create a new simulation instance
    let mut simulation = Simulation::new(config);
    
    // Record populations over time for analysis
    let mut prey_counts = Vec::new();
    let mut predator_counts = Vec::new();
    
    // Number of steps to run
    let steps = 1000;
    
    println!("\nRunning simulation for {} steps...", steps);
    
    // Measure performance
    let start_time = Instant::now();
    
    // Run the simulation for a fixed number of steps
    for step in 0..steps {
        // Update the simulation by one step
        simulation.update();
        
        // Record current population counts
        prey_counts.push(simulation.get_prey_count());
        predator_counts.push(simulation.get_predator_count());
        
        // Print current status every 50 steps
        if step % 50 == 0 || step == steps - 1 {
            println!(
                "Step {}: {} prey, {} predators", 
                step, 
                simulation.get_prey_count(),
                simulation.get_predator_count()
            );
        }
        
        // Exit early if one population goes extinct
        if simulation.get_prey_count() == 0 || simulation.get_predator_count() == 0 {
            println!("\nPopulation extinct at step {}!", step);
            break;
        }
    }
    
    // Calculate and print performance metrics
    let elapsed = start_time.elapsed();
    println!("\nSimulation completed in {:.2?}", elapsed);
    println!("Average time per step: {:.2?}", elapsed / steps as u32);
    
    // Print final statistics
    println!("\nFinal statistics:");
    println!("  Prey: {}", simulation.get_prey_count());
    println!("  Predators: {}", simulation.get_predator_count());
    
    // We could write the population data to a file for external plotting
    println!("\nPopulation data could be saved to a file for visualization.");
}