use std::error::Error;
pub mod simulation;
use crate::simulation::Simulation;

enum Action {
    Next,
    Previous,
    PrecomputeNext(usize),
    InvalidAction,
    Help,
    Quit,
}

fn get_action_from(string: &str) -> Option<Action> {
    let mut words = string.split_ascii_whitespace();

    if let Some(first_word) = words.next() {
        Some(match first_word.to_ascii_lowercase().as_str() {
            "n" | "next" => Action::Next,
            "p" | "prev" | "previous" => Action::Previous,
            "e" | "exp" | "expand" => get_number_of_steps_to_precompute(&mut words),
            "q" | "quit" => Action::Quit,
            "h" | "help" => Action::Help,
            _ => Action::InvalidAction,
        })
    } else {
        None
    }
}

fn get_number_of_steps_to_precompute(words: &mut std::str::SplitAsciiWhitespace) -> Action {
    if let Some(next_word) = words.next() {
        if let Some(how_many) = next_word.parse::<usize>().ok() {
            Action::PrecomputeNext(how_many)
        } else {
            Action::InvalidAction
        }
    } else {
        Action::InvalidAction
    }
}

const HELP_MESSAGE: &str = " [n] next                 - go forwards in simulation
 [p] prev                 - go backwards in simulation
 [e] expand <how_many>    - precalculate steps of simulation                       
 [h] help                 - print this list of commands
 [q] quit                 - quit the application";

fn main() -> Result<(), Box<dyn Error>> {
    let mut simulation = Simulation::load_from_file("input_big.txt")?;
    let mut default_action = Action::Next;

    simulation.print();
    loop {
        let mut should_print = true;
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line);

        match get_action_from(&line) {
            Some(action) => match action {
                Action::Next => {
                    default_action = Action::Next;
                    simulation.step_forwards();
                }
                Action::Previous => {
                    default_action = Action::Previous;
                    simulation.step_backwards();
                }
                Action::PrecomputeNext(how_many) => {
                    should_print = false;
                    println!("Precalculating {how_many} steps of simulaton...");
                    simulation.precalculate_next_n(how_many);
                    println!("Done!");
                }
                Action::InvalidAction => {
                    should_print = false;
                    println!("Invalid input. Type \"help\" for list of comands.");
                }
                Action::Help => {
                    should_print = false;
                    println!("{HELP_MESSAGE}");
                }
                Action::Quit => {
                    break;
                }
            },
            None => match default_action {
                Action::Next => {
                    simulation.step_forwards();
                }
                Action::Previous => {
                    simulation.step_backwards();
                }
                _ => {
                    unreachable!();
                }
            },
        };

        if should_print {
            simulation.print();
        }
    }

    Ok(())
}
