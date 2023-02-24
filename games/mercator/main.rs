extern crate mercator_lib;

use mercator_lib::game::{PlayerStrategy, Simulation};

fn main() {
    let player_strategies = [PlayerStrategy::Random; 3];
    let mut sim = Simulation::new(&player_strategies);

    while sim.step().is_some() {
        println!("STEP");
    }
}
