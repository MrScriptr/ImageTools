use core::{arch, error};
use std::{clone, env};

#[derive(Debug)]
enum Command {
    Convert(String),
}

#[derive(Debug)]
enum Error {
    InvalidCommand,
    NotEnoughParams
}

fn parseargs(args: Vec<String>) -> Result<Command, Error>  {
    if args.len() < 2 {
        return Err(Error::NotEnoughParams);
    }

    let path = args[0].clone();
    let command = args[1].clone().to_lowercase();

    match command.as_str() {
        "convert" => Ok(Command::Convert(path)),
        _ => Err(Error::InvalidCommand)
    }
}

fn runcommand(command: Command) {

}

fn printerror(error: Error) {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = parseargs(args);

    match command {
        Ok(command) => runcommand(command),
        Err(error) => printerror(error),
    }
}
