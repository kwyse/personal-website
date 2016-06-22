#[macro_use]
extern crate log;
extern crate log4rs;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate handlebars_iron;
extern crate hoedown;

use std::path::Path;
use iron::prelude::*;
use iron::AfterMiddleware;
use iron::status;
use router::{ Router, NoRoute };
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
    router.get("/blog", handle_blog_menu_page);
    router.get("/blog/:post", handle_blog_post_page);
    router.get("/projects", handle_projects_page);
    router.get("/contact", handle_contact_page);

    let mut chain = Chain::new(router);
    chain.link_after(template_engine);
    chain.link_after(PageNotFound);

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

enum MetadataField {
    Title,
    Author,
    Date,
    Path,
    Summary,
}

#[derive(Debug)]
struct Metadata {
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    path: Option<String>,
    summary: Option<String>,
}

use std::collections::HashMap;

#[derive(Debug)]
struct BlogPost {
    pub metadata: Metadata,
    pub body: String,
    // pub met: HashMap<MetadataField, String>,
    pub met2: HashMap<String, String>,
}

impl BlogPost {
    fn new() -> Self {
        BlogPost {
            metadata: Metadata::new(),
            body: String::new(),
            // met: HashMap::new(),
            met2: HashMap::new(),
        }
    }
}

impl Metadata {
    fn new() -> Self {
        Metadata {
            title: None,
            author: None,
            date: None,
            path: None,
            summary: None,
        }
    }
}

impl BlogPost {
    fn from_file<P: AsRef<Path>>(file_path: P) -> Result<BlogPost> {
        use std::fs::File;
        use std::io::BufReader;
        use std::io::BufRead;
        use std::io::Read;

        let file = try!(File::open(file_path));
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        let mut post = BlogPost::new();
        while try!(reader.read_line(&mut buffer)) > 0 {
            {
                let mut parts = buffer.splitn(2, ':');
                let first = parts.next().unwrap_or("");
                if first.len() > 0 && first.split_whitespace().count() == 1 {
                    let second = String::from(parts.next().unwrap_or("").trim());
                    post.met2.insert(String::from(first.to_lowercase()), second);
                } else {
                    break;
                }
            }
            buffer.clear();
        }

        reader.read_to_string(&mut buffer).unwrap_or(0);
        post.body = buffer;
        Ok(post)
    }
}

use std::io::Result;

fn get_file_contents<P: AsRef<Path>>(file_path: P) -> Result<String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}

fn get_blog_posts() -> Result<Vec<BlogPost>> {
    use std::fs::read_dir;
    use std::path::PathBuf;

    let mut posts: Vec<BlogPost> = Vec::new();
    let file_paths: Vec<PathBuf> = try!(read_dir("posts/")).map(|path| path.unwrap().path()).collect();
    for file_path in file_paths {
        // let contents = try!(get_file_contents(file_path));
        // parse_blog_post(file_path);
        posts.push(try!(BlogPost::from_file(file_path)));
    }

    info!("{:?}", posts);
    Ok(posts)
}

fn handle_blog_menu_page(_: &mut Request) -> IronResult<Response> {
    let posts = get_blog_posts().unwrap_or(Vec::new());
    let mut post_list: Vec<HashMap<String, String>> = Vec::new();

    for post in posts {
        post_list.push(post.met2);
    }

    let mut template_data: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    template_data.insert(String::from("posts"), post_list);

    Ok(Response::with((status::Ok, Template::new("blog_list", template_data))))
}

fn handle_blog_post_page(request: &mut Request) -> IronResult<Response> {
    use std::fs::File;
    use std::fs::read_dir;
    use std::path::Path;
    use std::collections::HashMap;
    use hoedown::{ Html, Markdown, Render, Buffer };
    use hoedown::renderer::html::Flags;

    let ref post_path = request.extensions.get::<Router>().unwrap().find("post").unwrap_or("/");
    let mut data = HashMap::new();

    let mut paths_to_content: HashMap<String, String> = HashMap::new();
    let posts = get_blog_posts().unwrap_or(Vec::new());

    for post in posts {
        let key = post.met2.get("path").unwrap().clone();
        paths_to_content.insert(key, post.body);
    }

    let not_found = String::from("Blog post not found");
    let content = paths_to_content.get(&String::from(*post_path)).unwrap_or(&not_found).clone();
    let c: &str = &content;
    let buffer: Buffer = Buffer::from(c);
    let post_as_markdown = Markdown::from(buffer);
    let mut html_renderer = Html::new(Flags::empty(), 0);
    let post_as_html = html_renderer.render(&post_as_markdown).to_str().expect("Converting post contents to string").to_string();

    data.insert(String::from("post"), post_as_html);
    Ok(Response::with((status::Ok, Template::new("blog_post", data))))
}

fn get_metadata(document: &mut hoedown::Markdown) -> ::std::collections::HashMap<String, String> {
    use std::io::BufRead;
    use std::io::BufReader;
    use std::collections::HashMap;
    let ref mut contents = document.contents;
    let ref mut reader = BufReader::new(contents);
    let mut metadata: HashMap<String, String> = HashMap::new();
    for l in reader.lines() {
        let line = l.expect("Iterating through metadata");
        if line.is_empty() {
            break;
        } else {
            let mut key_value = line.split(':');
            let key = key_value.next().expect("Assigning key").to_lowercase();
            let value = key_value.next().expect("Assigning value").trim();
            metadata.insert(key.to_string(), value.to_string());
        }
    }
    // info!("REMAINING {:?}", contents.to_str().unwrap());
    // debug!(" METADATA IS {:?}", metadata);
    metadata
}

fn handle_projects_page(_: &mut Request) -> IronResult<Response> {
    unimplemented!();
}

fn handle_contact_page(_: &mut Request) -> IronResult<Response> {
    unimplemented!();
}

struct PageNotFound;

impl AfterMiddleware for PageNotFound {
    fn catch(&self, _: &mut Request, error: IronError) -> IronResult<Response> {
        info!("Page not found!");

        if let Some(_) = error.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Page not found")))
        } else {
            Err(error)
        }
    }
}
