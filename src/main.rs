use std::time::Instant;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::env;
use life_game::{Simulation, SimulationConfig};

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut steps = 1000;
    let mut output_file = None;
    
    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" | "--steps" => {
                if i + 1 < args.len() {
                    if let Ok(n) = args[i+1].parse() {
                        steps = n;
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            },
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i+1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            },
            _ => i += 1,
        }
    }
    
    // Create a simulation configuration with custom parameters for more dynamics
    let config = SimulationConfig {
        width: 100,
        height: 100,
        prey_reproduction_rate: 0.3,
        prey_moving_factor: 0.7,     // Higher mobility
        predator_hunting_factor: 0.6, // Better hunters
        predator_reproduction_rate: 0.2, // Slower reproduction
        predator_death_after: 20,     // Starve faster
        nb_prey_init: 150,
        nb_predator_init: 50,
        ..Default::default()
    };
    
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
    let mut prey_counts = Vec::with_capacity(steps);
    let mut predator_counts = Vec::with_capacity(steps);
    
    println!("\nRunning simulation for {} steps...", steps);
    
    // Measure performance
    let start_time = Instant::now();
    
    // Run the simulation for the specified number of steps
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
    
    // Write population data to CSV file if requested
    if let Some(filename) = output_file {
        if let Ok(mut file) = File::create(Path::new(&filename)) {
            // Write CSV header
            writeln!(file, "step,prey,predators").unwrap();
            
            // Write data for each step
            for (i, (prey, predator)) in prey_counts.iter().zip(predator_counts.iter()).enumerate() {
                writeln!(file, "{},{},{}", i, prey, predator).unwrap();
            }
            
            println!("\nPopulation data saved to '{}'", filename);
            println!("You can use this data for visualization with external tools.");
        } else {
            println!("\nFailed to create output file '{}'", filename);
        }
    } else {
        println!("\nUse -o <filename.csv> to save population data for visualization.");
    }
}