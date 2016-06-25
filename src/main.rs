#[macro_use]
extern crate log;
extern crate log4rs;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron;
extern crate hoedown;
extern crate rustc_serialize;

mod blog;
mod route;

use iron::prelude::*;
use iron::AfterMiddleware;
use iron::middleware::Handler;
use mount::Mount;
use router::Router;
use handlebars_iron::HandlebarsEngine;

const PORT: u16 = 42451;

fn main() {
    let routes = add_routes();
    let mounts = add_mounts(routes);
    let mut chain = Chain::new(mounts);

    let templates = add_templates();
    chain.link_after(templates);
    chain.link_after(PageNotFound);

    log4rs::init_file("config/log4rs.yml", Default::default()).expect("Attempting to intialize logger");

    let url: &str = &format!("localhost:{}", PORT);
    info!("Server started on {}", url);
    Iron::new(chain).http(url).unwrap();
}

fn add_routes() -> Router {
    use route::*;

    let mut router = Router::new();
    router.get("/", handle_landing_page);
    router.get("/blog", handle_blog_list_page);
    router.get("/blog/:post", handle_blog_post_page);
    router
}

fn add_mounts<H: Handler>(router: H) -> Mount {
    use std::path::Path;
    use staticfile::Static;

    let mut mounts = Mount::new();
    mounts.mount("/", router);
    mounts.mount("/static", Static::new(Path::new("static")));
    mounts
}

fn add_templates() -> HandlebarsEngine {
    use handlebars_iron::DirectorySource;

    let mut template_engine = HandlebarsEngine::new();
    template_engine.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    template_engine.reload().expect("Attempting to load Handlebars tempaltes");
    template_engine
}

struct PageNotFound;

impl AfterMiddleware for PageNotFound {
    fn catch(&self, _: &mut Request, error: IronError) -> IronResult<Response> {
        use std::error::Error;
        use iron::status;
        use router::NoRoute;

        if let Some(_) = error.error.downcast::<NoRoute>() {
            info!("Page not found: {}", error.description());
            Ok(Response::with((status::NotFound, "Page not found")))
        } else {
            Err(error)
        }
    }
}
