extern crate diesel;
extern crate dotenv;
extern crate sass_rs;

use diesel::{Connection, migrations};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use sass_rs::sass_context::SassFileContext;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let compiled_css = SassFileContext::new("app/sass/main.scss")
        .compile()
        .expect("Attempting to compile input SCSS");

    let mut output_file = File::create("static/main.css").expect("Attempting to create output CSS file");
    output_file.write(compiled_css.as_bytes()).expect("Attempting to write to output CSS file");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    migrations::run_pending_migrations(&conn).ok();
}
