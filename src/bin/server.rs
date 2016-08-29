extern crate staticfile;
extern crate personal_website;

use std::path::Path;
use staticfile::Static;
use personal_website::app::App;
use personal_website::route::*;

#[cfg(not(feature = "live-reload"))]
fn main() {
    App::new("127.0.0.1")
        .add_get_route("/", handle_landing_page)
        .add_get_route("/about", handle_about_page)
        .add_get_route("/blog", handle_blog_list_page)
        .add_get_route("/blog/:post", handle_blog_post_page)
        .add_mount("/static", Static::new(Path::new("./static")))
        .add_mount("/app", Static::new(Path::new("./app")))
        .add_mount("/bower_components", Static::new(Path::new("./bower_components")))
        .set_template_dir("./templates")
        .build_and_run();
}

#[cfg(feature = "live-reload")]
fn main() {
    App::new("127.0.0.1")
        .add_get_route("/", handle_landing_page)
        .add_get_route("/about", handle_about_page)
        .add_get_route("/blog", handle_blog_list_page)
        .add_get_route("/blog/:post", handle_blog_post_page)
        .add_mount("/static", Static::new(Path::new("./static")))
        .add_mount("/app", Static::new(Path::new("./app")))
        .add_mount("/bower_components", Static::new(Path::new("./bower_components")))
        .set_template_dir("./templates")
        .build_and_run_reloadable();
}
