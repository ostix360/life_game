use criterion::{criterion_group, criterion_main, Criterion};
use life_game::simulation::Simulation;

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
        (w*h / 4) as u32,
        (w*h / 4) as u32,
    );
    sim.init_grid();
    sim
}

pub fn bench_sim(c: &mut Criterion) {
    let mut sim = create_test_simulation(102, 102);
    sim.init_grid();
    sim.init_simulation();
    c.bench_function("sim 102x102", |b| {
            b.iter(|| { 
                sim.simulate();
            })
        }
    );
    let mut sim = create_test_simulation(501, 501);
    sim.init_grid();
    sim.init_simulation();
    c.bench_function("sim 501x501", |b| {
            b.iter(|| { 
                sim.simulate();
            })
        }
    );
    
    let mut sim = create_test_simulation(1002, 1002);
    sim.init_grid();
    sim.init_simulation();
    c.bench_function("sim 1002x1002", |b| {
            b.iter(|| { 
                sim.simulate();
            })
        }
    );
}

criterion_group!(benches, bench_sim);
criterion_main!(benches);