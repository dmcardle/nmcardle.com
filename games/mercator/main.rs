extern crate mercator_lib;

use mercator_lib::game::Simulation;

fn main() {
    const NUM_PLAYERS: usize = 3;
    let mut sim = Simulation::new(NUM_PLAYERS);

    while sim.step().is_some() {
        println!("STEP");
    }
}
