extern crate mercator_lib;

use mercator_lib::game::Simulation;

fn main() {
    const NUM_PLAYERS : usize = 3;
    let sim = Simulation::new(NUM_PLAYERS);
    println!("HELLO!");
}
