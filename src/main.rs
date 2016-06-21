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
    router.get("/blog", handle_blog_menu_page);
    router.get("/blog/:post", handle_blog_post_page);
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

fn handle_blog_menu_page(_: &mut Request) -> IronResult<Response> {
    use std::fs::File;
    use std::fs::read_dir;
    use std::path::Path;
    use std::collections::HashMap;
    use hoedown::{ Html, Markdown, Render };
    use hoedown::renderer::html::Flags;

    let posts = read_dir(Path::new("posts/")).expect("Reading dir");
    let mut posts_to_render: Vec<HashMap<String, String>> = Vec::new();

    for directory_entry in posts {
        let post = directory_entry.expect("Iterating through directory entries");
        let post_os_path = post.path();
        let post_path = post_os_path.as_path();

        let file = File::open(post_path).expect("Reading post from disk");
        let mut post_as_markdown = Markdown::read_from(file);
        let metadata = get_metadata(&mut post_as_markdown);
        let metadata_entry_count = metadata.len();

        let contents_with_metadata = post_as_markdown.contents.to_str().unwrap().to_string();
        let mut contents_and_metadata = contents_with_metadata.splitn(2, "\n\n");
        // let metadata_only = contents_and_metadata.next().unwrap();
        let contents_only = contents_and_metadata.next().unwrap();
        post_as_markdown.contents = hoedown::Buffer::from(contents_only);

        // info!("NEW META {:?}\nAnd COUNT {:?}", metadata, metadata_entry_count);

        let post_title = post.file_name().into_string().expect("Converting post title to string");
        let mut html_renderer = Html::new(Flags::empty(), 0);
        let post_as_html = html_renderer.render(&post_as_markdown).to_str().expect("Converting post contents to string").to_string();

        let mut post_object = HashMap::new();
        post_object.insert(String::from("title"), metadata.get(&String::from("title")).expect("title").clone());
        post_object.insert(String::from("author"), metadata.get(&String::from("author")).expect("author").clone());
        post_object.insert(String::from("date"), metadata.get(&String::from("date")).expect("date").clone());
        post_object.insert(String::from("path"), metadata.get(&String::from("path")).expect("path").clone());
        post_object.insert(String::from("summary"), metadata.get(&String::from("summary")).expect("summary").clone());
        post_object.insert("body".to_string(), post_as_html);
        posts_to_render.push(post_object);

    }

    let mut template_data: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    template_data.insert("posts".to_string(), posts_to_render);
    // debug!("Rendering: {:?}", template_data);

    Ok(Response::with((status::Ok, Template::new("blog_list", template_data))))
}

fn handle_blog_post_page(request: &mut Request) -> IronResult<Response> {
    use std::fs::File;
    use std::fs::read_dir;
    use std::path::Path;
    use std::collections::HashMap;
    use hoedown::{ Html, Markdown, Render };
    use hoedown::renderer::html::Flags;

    let ref post = request.extensions.get::<Router>().unwrap().find("post").unwrap_or("/");
    let mut data = HashMap::new();

    let posts = read_dir(Path::new("posts/")).expect("Reading dir");
    let mut paths_to_content: HashMap<String, String> = HashMap::new();

    for directory_entry in posts {
        let post = directory_entry.expect("Iterating through directory entries");
        let post_os_path = post.path();
        let post_path = post_os_path.as_path();

        let file = File::open(post_path).expect("Reading post from disk");
        let mut post_as_markdown = Markdown::read_from(file);
        let metadata = get_metadata(&mut post_as_markdown);

        let contents_with_metadata = post_as_markdown.contents.to_str().unwrap().to_string();
        let mut contents_and_metadata = contents_with_metadata.splitn(2, "\n\n");
        let metadata_only = contents_and_metadata.next().unwrap();
        let contents_only = contents_and_metadata.next().unwrap();
        post_as_markdown.contents = hoedown::Buffer::from(contents_only);

        let mut html_renderer = Html::new(Flags::empty(), 0);
        let post_as_html = html_renderer.render(&post_as_markdown).to_str().expect("Converting post contents to string").to_string();
        paths_to_content.insert(metadata.get(&String::from("path")).expect("path").clone(), post_as_html);
    }

    let not_found = String::from("Not found");
    let content = paths_to_content.get(&String::from(*post)).unwrap_or(&not_found).clone();
    data.insert(String::from("post"), content);
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
