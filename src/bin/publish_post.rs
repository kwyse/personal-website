extern crate personal_website;

use std::io::stdin;
use personal_website::db::{establish_connection, publish_post};

fn main() {
    println!("Enter ID:");

    let mut id_as_string = String::new();
    stdin().read_line(&mut id_as_string).unwrap();
    let id = id_as_string[..(id_as_string.len() - 1)]
        .parse::<i32>()
        .expect("Error parsing ID");

    let conn = establish_connection();
    publish_post(&conn, id);
    println!("Successfully pubished post with ID {}", id);
}
