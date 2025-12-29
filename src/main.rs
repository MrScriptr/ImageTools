use core::{arch, error};
use std::{clone, env, result};

use image::{ImageError, open};

#[derive(Debug)]
enum Command {
    Convert(String),
}

#[derive(Debug)]
enum Error {
    InvalidCommand,
    NotEnoughParams,
    Image(ImageError)
}

enum Output {
    Ok,
    Error(Error)
}

fn parseargs(args: Vec<String>) -> Result<Command, Error>  {
    if args.len() < 2 {
        return Err(Error::NotEnoughParams);
    }

    let path = args[1].clone();
    let command = args[2].clone().to_lowercase();

    match command.as_str() {
        "convert" => Ok(Command::Convert(path)),
        _ => Err(Error::InvalidCommand)
    }
}

fn convert(path: String) -> Output {
    println!("Converting");
    let openedimage = image::open(&path);
    match openedimage {
        Ok(image) => {
            image.save("output.png");
            return Output::Ok;
        },
        Err(error) => return Output::Error(Error::Image(error)),
    }
}

fn runcommand(command: Command) -> Output {
    match command {
        Command::Convert(path) => convert(path),
    }
}

fn printerror(error: Error) {
    let output = match error {
        Error::InvalidCommand => "Please use a valid command",
        Error::NotEnoughParams => "Please use more parameters",
        Error::Image(err) => "Error parsing Image" 
    };
    println!("{}", output);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = parseargs(args);

    match command {
        Ok(command) => {
            match runcommand(command) {
                Output::Ok => {},
                Output::Error(err) => {
                    printerror(err);
                }
            }
        },
        Err(error) => printerror(error),
    }
}
