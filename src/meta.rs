#![allow(dead_code)]

use std::process;

pub enum CommandResult {
    Success,
    UnrecognizedCommand,
}

pub fn do_command(input: &str) -> CommandResult {
    let v: Vec<&str> = input.split(" ").collect();

    match &v[..] {
        [".exit"] => {
            println!("Exiting.");
            process::exit(0);
        }
        _ => return CommandResult::UnrecognizedCommand,
    }
}
