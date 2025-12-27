use std::{clone, env};

enum Command {
    Convert(String),
}

enum Error {
    InvalidCommand
}

fn parseargs(args: Vec<String>) -> Result<Command, Error>  {
    let path = args[1].clone();
    let command = args[2].clone().to_lowercase();

    match command.as_str() {
        "convert" => Ok(Command::Convert(path)),
        _ => Err(Error::InvalidCommand)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
}
