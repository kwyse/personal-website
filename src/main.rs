#![feature(custom_derive, custom_attribute, plugin)]

#![plugin(diesel_codegen, dotenv_macros, serde_macros)]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate diesel;
#[macro_use] extern crate log;

extern crate fern;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron;
extern crate hoedown;
extern crate rustc_serialize;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

mod blog;
mod db;
mod route;

use iron::prelude::*;
use iron::AfterMiddleware;
use iron::middleware::Handler;
use mount::Mount;
use router::Router;
use handlebars_iron::HandlebarsEngine;

#[cfg(feature = "live-reload")]
use handlebars_iron::Watchable;

const DEFAULT_PORT: &'static str = "3000";

#[cfg(not(feature = "live-reload"))]
fn main() {
    init_logger();

    let routes = add_routes();
    let mounts = add_mounts(routes);
    let mut chain = Chain::new(mounts);

    info!("Template live reload disabled");
    let templates = add_templates();
    chain.link_after(templates);
    chain.link_after(PageNotFound);

    let url: &str = &format!("0.0.0.0:{}", get_server_port());
    info!("Server started on {}", url);
    Iron::new(chain).http(url).unwrap();
}

#[cfg(feature = "live-reload")]
fn main() {
    use std::sync::Arc;

    init_logger();

    let routes = add_routes();
    let mounts = add_mounts(routes);
    let mut chain = Chain::new(mounts);

    info!("Template live reload enabled");
    let templates = add_templates();
    let templates_ref = Arc::new(templates);
    templates_ref.watch("templates/");

    chain.link_after(templates_ref);
    chain.link_after(PageNotFound);

    let url: &str = &format!("0.0.0.0:{}", get_server_port());
    info!("Server started on {}", url);
    Iron::new(chain).http(url).unwrap();
}

fn init_logger() {
    use chrono::UTC;
    use fern::{ DispatchConfig, OutputConfig };
    use log::LogLevelFilter;

    let logger_config = DispatchConfig {
        format: Box::new(|msg, level, _| {
            format!("{} [{}] | {}", UTC::now().format("[%Y-%m-%d %H:%M:%S]"), level, msg)
        }),
        output: vec![OutputConfig::stdout(), OutputConfig::file("output.log")],
        level: LogLevelFilter::Trace,
    };

    fern::init_global_logger(logger_config, LogLevelFilter::Info).expect("Attempting to initialize global logger");
}

fn add_routes() -> Router {
    use route::*;

    let mut router = Router::new();
    router.get("/", handle_landing_page);
    router.get("/about", handle_about_page);
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
    mounts.mount("/app", Static::new(Path::new("app")));
    mounts
}

fn add_templates() -> HandlebarsEngine {
    use handlebars_iron::DirectorySource;

    let mut template_engine = HandlebarsEngine::new();
    template_engine.add(Box::new(DirectorySource::new("templates/", ".hbs")));
    template_engine.reload().expect("Attempting to load Handlebars templates");
    template_engine
}

fn get_server_port() -> u16 {
    use std::env;
    use dotenv::dotenv;

    dotenv().ok();

    env::var("PORT")
        .unwrap_or_else(|_| {
            info!("PORT is not set, defaulting to {}", DEFAULT_PORT);
            String::from(DEFAULT_PORT)
        }).parse().expect("Attempting to parse server port number")
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
