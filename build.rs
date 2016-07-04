extern crate sass_rs;

use std::fs::File;
use std::io::Write;
use sass_rs::sass_context::SassFileContext;

fn main() {
    compile_scss();
}

fn compile_scss() {
    let compiled_css = SassFileContext::new("app/sass/main.scss")
        .compile()
        .expect("Attempting to compile input SCSS");

    let mut output_file = File::create("static/main.css").expect("Attempting to create output CSS file");
    output_file.write(compiled_css.as_bytes()).expect("Attempting to write to output CSS file");
}
