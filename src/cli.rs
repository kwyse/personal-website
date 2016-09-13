//! Functions for CLI utilities

use clap::ArgMatches;
use diesel::result;
use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::num;
use std::path::Path;

pub fn apply(matches: ArgMatches) -> Result<(), CliError> {
    match matches.subcommand() {
        ("update", Some(sub_matches)) => update(sub_matches),
        _ => Ok(()) // clap will prevent unknown subcommand being used
    }
}

fn update(matches: &ArgMatches) -> Result<(), CliError> {
    let id = try!(matches.value_of("ID").unwrap().parse::<i32>());
    let body = matches.value_of("FILE").unwrap();
    update_body(id, body)
}

fn update_body<P: AsRef<Path>>(id: i32, filename: P) -> Result<(), CliError> {
    use db::{establish_connection, update_body};

    let conn = establish_connection();
    let body = try!(string_from_file(filename));
    update_body(&conn, id, &body).map(|_| ()).map_err(CliError::Database) // TODO: why is this needed?
}

fn string_from_file<P: AsRef<Path>>(filename: P) -> Result<String, CliError> {
    use std::io::Read;

    let mut file = try!(File::open(filename));
    let mut string = String::new();
    try!(file.read_to_string(&mut string));
    Ok(string)
}

#[derive(Debug)]
pub enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Database(result::Error),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

impl From<result::Error> for CliError {
    fn from(err: result::Error) -> CliError {
        CliError::Database(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Parse(ref err) => err.fmt(f),
            CliError::Database(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => err.description(),
            CliError::Database(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
            CliError::Database(ref err) => Some(err),
        }
    }
}
