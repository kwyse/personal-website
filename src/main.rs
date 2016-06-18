#[macro_use]
extern crate log;
extern crate log4rs;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron;

use std::path::Path;
use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use staticfile::Static;
use handlebars_iron::{ HandlebarsEngine, DirectorySource, Template };

fn main() {
    let mut template_engine = HandlebarsEngine::new();
    template_engine.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    template_engine.reload().expect("Loading templates");

    let mut router = Router::new();
    router.get("/", handle_landing_page);
    router.get("/about", handle_about_page);
    router.get("/blog", handle_blog_page);
    router.get("/projects", handle_projects_page);
    router.get("/contact", handle_contact_page);

    let mut chain = Chain::new(router);
    chain.link_after(template_engine);

    let mut mounts = Mount::new();
    mounts.mount("/", chain);
    mounts.mount("/static", Static::new(Path::new("static")));
    mounts.mount("/app/js", Static::new(Path::new("app/js")));

    log4rs::init_file("config/log4rs.yml", Default::default()).expect("Intializing log4rs");
    info!("Running on port 42451");
    Iron::new(mounts).http("localhost:42451").unwrap();
}

fn handle_landing_page(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Template::new("landing", ()))))
}

fn handle_about_page(_: &mut Request) -> IronResult<Response> {
    unimplemented!();
}

fn handle_blog_page(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Template::new("blog", ()))))
}

fn handle_projects_page(_: &mut Request) -> IronResult<Response> {
    unimplemented!();
}

fn handle_contact_page(_: &mut Request) -> IronResult<Response> {
    unimplemented!();
}
