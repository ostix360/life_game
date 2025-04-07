use criterion::{black_box, criterion_group, criterion_main, Criterion};
use life_game::simulation::{Simulation, get_cell};

fn create_test_simulation(w: i32, h: i32) -> Simulation {
    let mut sim = Simulation::new(
        w,
        h,
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

pub fn bench_sim(c: &mut Criterion) {
    let mut sim = create_test_simulation(100, 100);
    sim.init_grid();
    sim.init_simulation();
    c.bench_function("sim 100x100", |b| {
            b.iter(|| { 
                sim.simulate();
            })
        }
    );
    let mut sim = create_test_simulation(500, 500);
    sim.init_grid();
    sim.init_simulation();
    c.bench_function("sim 500x500", |b| {
            b.iter(|| { 
                sim.simulate();
            })
        }
    );
    
    let mut sim = create_test_simulation(1000, 1000);
    sim.init_grid();
    sim.init_simulation();
    c.bench_function("sim 1000x1000", |b| {
            b.iter(|| { 
                sim.simulate();
            })
        }
    );
}

criterion_group!(benches, bench_sim);
criterion_main!(benches);