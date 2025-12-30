use core::{arch, error};
use std::{clone, env, result};

use image::{ImageError, open};

#[derive(Debug)]
enum Operation {
    Convert,
}

struct Command {
    Operation: Operation,
    Source: String,
    Output: String,
    Args: Vec<String>
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
    if args.len() < 3 {
        return Err(Error::NotEnoughParams);
    }

    let path = args[1].clone();
    let outputpath = args[2].clone();
    let command = args[3].clone().to_lowercase();

    match command.as_str() {
        "convert" => Ok(Command {
            Operation: Operation::Convert,
            Source: path,
            Output: outputpath,
            Args: args
        }),
        _ => Err(Error::InvalidCommand)
    }
}

fn convert(command: Command) -> Output {
    println!("Converting");
    let openedimage = image::open(&command.Source);
    match openedimage {
        Ok(image) => {
            image.save(command.Output);
            return Output::Ok;
        },
        Err(error) => return Output::Error(Error::Image(error)),
    }
}

fn runcommand(command: Command) -> Output {
    match &command.Operation {
        Operation::Convert => convert(command),
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
