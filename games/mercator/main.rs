extern crate mercator_lib;

use std::io::Write;

use mercator_lib::game::{PlayerStrategy, Simulation};

#[derive(Debug)]
enum Error {
    SimulationError(String),
    IoError(String),
}

enum UserChoice {
    Step,
    Run,
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
        "s\n" | "\n" => Ok(UserChoice::Step),
        "r\n" => Ok(UserChoice::Run),
        "q\n" => Ok(UserChoice::Quit),
        _ => {
            println!("Options: [q]uit, [r]un, or [s]tep (default=s).");
            prompt()
        }
    }
}

fn main() -> Result<(), Error> {
    let player_strategies = [PlayerStrategy::Random; 3];
    let mut sim = Simulation::new(&player_strategies);
    let mut run_mode = false;
    loop {
        println!("{}", sim);
        sim.step().map_err(Error::SimulationError)?;
        if run_mode {
            continue;
        }
        match prompt()? {
            UserChoice::Step => {},
            UserChoice::Run => { run_mode = true;},
            UserChoice::Quit => return Ok(()),
        }
    }
    Ok(())
}
