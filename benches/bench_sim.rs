use criterion::{criterion_group, criterion_main, Criterion};
use life_game::{Simulation, SimulationConfig};

fn create_test_simulation(w: usize, h: usize) -> Simulation {
    let config = SimulationConfig {
        width: w,
        height: h,
        prey_reproduction_rate: 0.5,
        prey_moving_factor: 0.5,     // Higher mobility
        predator_hunting_factor: 0.6, // Better hunters
        predator_reproduction_rate: 0.5, // Slower reproduction
        predator_death_after: 35,     // Starve faster
        nb_prey_init: w * h / 4,
        nb_predator_init: w * h / 4,
        ..Default::default()
    };
    let mut sim = Simulation::new(config);
    sim
}

pub fn bench_sim(c: &mut Criterion) {
    let mut sim = create_test_simulation(102, 102);
    c.bench_function("sim 102x102", |b| {
        b.iter(|| {
            sim.update();
        })
    }
    );
    let mut sim = create_test_simulation(501, 501);
    c.bench_function("sim 501x501", |b| {
        b.iter(|| {
            sim.update();
        })
    }
    );

    let mut sim = create_test_simulation(1002, 1002);
    c.bench_function("sim 1002x1002", |b| {
        b.iter(|| {
            sim.update();
        })
    }
    );
}

criterion_group!(benches, bench_sim);
criterion_main!(benches);