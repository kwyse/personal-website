extern crate personal_website;
extern crate chrono;

use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;
use chrono::Local;
use personal_website::db::models::NewBlogPost;
use personal_website::db::{create_post, establish_connection};

fn main() {
    let title = prompt_input("title");
    let summary = prompt_input("summary");
    let tags = prompt_input("tags").split_whitespace().map(str::to_string).collect();

    let file_name = prompt_input("Markdown file name");
    let file_path = Path::new(&file_name);

    let mut body_content = File::open(file_path).unwrap();
    let mut body = String::new();
    body_content.read_to_string(&mut body).unwrap();

    let created = Local::today().naive_local();
    let url = file_path.file_stem().unwrap().to_str().unwrap().replace("_", "-").to_lowercase();

    let new_post = NewBlogPost {
        title: title,
        created: created,
        published: false,
        url: url,
        summary: summary,
        body: body,
        tags: tags,
    };

    let conn = establish_connection();
    let added_post = create_post(&conn, new_post);
    println!("Successfully added new post to database (ID: {})", added_post.id);
}

fn prompt_input(input_field: &str) -> String {
    println!("Enter {}:", input_field);

    let mut field = String::new();
    stdin().read_line(&mut field).unwrap();
    field[..(field.len() - 1)].to_string()
}
