#[macro_use]
extern crate clap;
extern crate personal_website;

use std::error::Error;

use personal_website::cli::apply;

fn main() {
    let matches = clap_app!(postutil =>
        (version: "0.1")
        (author: "Krishan Wyse <kwysek@gmail.com>")
        (about: "Manage blog posts")
        (@subcommand update =>
            (about: "Updates the body of an existing blog post")
            (@arg ID: +required "The ID of the post to update")
            (@arg FILE: +required "The file containing the body contents")
        )
    ).get_matches();

    apply(matches).unwrap_or_else(|err| println!("Error executing command: {}", err.description()));
}
