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
        ("create", Some(sub_matches)) => create(sub_matches),
        ("update", Some(sub_matches)) => update(sub_matches),
        _ => Ok(()) // clap will prevent unknown subcommand being used
    }
}

fn create(matches: &ArgMatches) -> Result<(), CliError> {
    use chrono::Local;
    use db;

    let title = matches.value_of("TITLE").unwrap().trim().to_string();
    let body_file = matches.value_of("FILE").unwrap();
    let tags = get_optionals("tag", matches);
    let summary = get_optional("summary", matches);
    let publish = matches.is_present("publish");

    let url = title.replace(" ", "-").to_lowercase();
    let created = Local::today().naive_local();
    let body = try!(string_from_file(body_file));

    let new_post = db::models::NewBlogPost {
        title: title,
        created: created,
        published: publish,
        url: url,
        summary: summary,
        body: body,
        tags: tags,
    };

    let conn = db::establish_connection();
    let added_post = db::create_post(&conn, new_post);
    println!("Successfully added new post to database (ID: {})", added_post.id);

    Ok(())
}

fn get_optional(attribute: &str, matches: &ArgMatches) -> String {
    let value = matches.value_of(attribute).unwrap_or_default();
    if value.is_empty() {
        println!("warning: {} was not supplied", attribute);
    }

    value.to_string()
}

fn get_optionals(attribute: &str, matches: &ArgMatches) -> Vec<String> {
    if let Some(args) = matches.values_of(attribute) {
        args.map(str::to_string).collect::<Vec<String>>()
    } else {
        println!("warning: {} was not supplied", attribute);
        Vec::new()
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
