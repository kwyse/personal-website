//! Entry point for the web app

use router::Router;
use mount::Mount;
use iron::{Chain, Handler};
use handlebars_iron::HandlebarsEngine;

pub struct App {
    host: String,
    routes: Router,
    mounts: Mount,
    template_dir: String,
}

impl App {
    pub fn new(host: &str) -> Self {
        init_logger();

        App {
            host: host.to_string(),
            routes: Router::new(),
            mounts: Mount::new(),
            template_dir: String::new(),
        }
    }

    pub fn add_get_route<P: AsRef<str>, H: Handler>(&mut self, path: P, handler: H) -> &mut Self {
        self.routes.get(&path, handler, &path);
        self
    }

    pub fn add_mount<H: Handler>(&mut self, path: &str, handler: H) -> &mut Self {
        self.mounts.mount(path, handler);
        self
    }

    pub fn set_template_dir(&mut self, directory: &str) -> &mut Self {
        self.template_dir = directory.to_string();
        self
    }

    #[cfg(not(feature = "live-reload"))]
    pub fn build_and_run(&mut self) {
        self.build(|mounts, template_engine, _| {
            info!("Template live reloading disabled");

            let mut chain = Chain::new(mounts);
            chain.link_after(template_engine);
            chain
        });
    }

    #[cfg(feature = "live-reload")]
    pub fn build_and_run_reloadable(&mut self) {
        use std::sync::Arc;
        use handlebars_iron::Watchable;

        self.build(|mounts, template_engine, template_dir| {
            info!("Template live reloading enabled");

            let templates_ref = Arc::new(template_engine);
            templates_ref.watch(template_dir);

            let mut chain = Chain::new(mounts);
            chain.link_after(templates_ref);
            chain
        });
    }

    fn build<F>(&mut self, chain_func: F) where F: Fn(Mount, HandlebarsEngine, &String) -> Chain {
        use std::mem::swap;
        use iron::Iron;
        use handlebars_iron::DirectorySource;

        let mut routes = Router::new();
        swap(&mut routes, &mut self.routes);

        let mut mounts = Mount::new();
        swap(&mut mounts, &mut self.mounts);
        mounts.mount("/", routes);

        let mut template_engine = HandlebarsEngine::new();
        template_engine.add(Box::new(DirectorySource::new(&self.template_dir, ".hbs")));
        template_engine.reload().expect("Attempting to load Handlebars templates");

        let chain = chain_func(mounts, template_engine, &self.template_dir);

        let url: &str = &format!("{}:{}", self.host, get_server_port());
        info!("Server started on {}", url);
        Iron::new(chain).http(url).unwrap();
    }
}

fn init_logger() {
    use chrono::UTC;
    use fern::{DispatchConfig, OutputConfig, init_global_logger};
    use log::LogLevelFilter;

    let logger_config = DispatchConfig {
        format: Box::new(|msg, level, _| {
            format!("{} [{}] | {}", UTC::now().format("[%Y-%m-%d %H:%M:%S]"), level, msg)
        }),
        output: vec![OutputConfig::stdout(), OutputConfig::file("output.log")],
        level: LogLevelFilter::Trace,
    };

    init_global_logger(logger_config, LogLevelFilter::Info).expect("Attempting to initialize global logger");
}

fn get_server_port() -> u16 {
    use std::env;
    use dotenv::dotenv;

    dotenv().ok();

    let default_port = "3000";
    env::var("PORT")
        .unwrap_or_else(|_| {
            info!("PORT is not set, defaulting to {}", default_port);
            default_port.to_string()
        }).parse().expect("Attempting to parse server port number")
}
