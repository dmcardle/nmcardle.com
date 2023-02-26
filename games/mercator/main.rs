extern crate mercator_lib;

use std::io::Write;

use mercator_lib::game::{PlayerStrategy, Simulation};

#[derive(Debug)]
enum Error {
    SimulationError(String),
    IoError(String),
}

enum UserChoice {
    Continue,
    Quit,
}

fn prompt() -> Result<UserChoice, Error> {
    print!(">> ");
    std::io::stdout()
        .flush()
        .map_err(|_| Error::IoError("Failed to flush stdout".to_string()))?;
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .map_err(|_| Error::IoError("Failed to read stdin".to_string()))?;

    match buf.as_str() {
        "\n" => Ok(UserChoice::Continue),
        "q\n" => Ok(UserChoice::Quit),
        _ => {
            println!("Type 'q' to quit, or press enter to continue.");
            prompt()
        }
    }
}

fn main() -> Result<(), Error> {
    let player_strategies = [PlayerStrategy::Random; 3];
    let mut sim = Simulation::new(&player_strategies);
    loop {
        println!("{}", sim);
        sim.step().map_err(Error::SimulationError)?;
        match prompt()? {
            UserChoice::Continue => {}
            UserChoice::Quit => return Ok(()),
        }
    }
    Ok(())
}
