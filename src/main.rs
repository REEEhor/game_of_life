use std::error::Error;
pub mod simulation;
use crate::simulation::Simulation;

fn wait_for_enter() {
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut simulation = Simulation::load_from_file("input.txt")?;

    loop {
        simulation.step_forwards();
        simulation.print();
        wait_for_enter();
    }
}
