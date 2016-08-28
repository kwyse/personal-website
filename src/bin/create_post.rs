extern crate personal_website;
extern crate chrono;

use std::io::{stdin, Read};
use std::fs::File;
use chrono::Local;
use personal_website::db::models::NewBlogPost;
use personal_website::db::create_post;

fn main() {
    let title = prompt_input("title");
    let summary = prompt_input("summary");
    let tags = prompt_input("tags").split_whitespace().map(str::to_string).collect();
    let body_file_name = prompt_input("Markdown file name");

    let mut body_file = File::open(body_file_name).unwrap();
    let mut body = String::new();
    body_file.read_to_string(&mut body).unwrap();

    let created = Local::today().naive_local();
    let url = title.replace(" ", "-").to_lowercase();

    let new_post = NewBlogPost {
        title: title,
        created: created,
        url: url,
        summary: summary,
        body: body,
        tags: tags,
    };

    let added_post = create_post(new_post);
    println!("Successfully added new post to database (ID: {})", added_post.id);
}

fn prompt_input(input_field: &str) -> String {
    println!("Enter {}:", input_field);

    let mut field = String::new();
    stdin().read_line(&mut field).unwrap();
    field[..(field.len() - 1)].to_string()
}
