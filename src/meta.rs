use std::process;

pub enum CommandResult {
    Success,
    UnrecognizedCommand,
}

pub fn do_command(input: &str) -> CommandResult {
    use crate::meta::CommandResult::*;
    let v: Vec<&str> = input.split(" ").collect();

    match &v[..] {
        [".exit"] => {
            println!("Exiting.");
            process::exit(0);
        }
        _ => return UnrecognizedCommand,
    }
}
