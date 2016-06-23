//! Provides routes for main application

use std::collections::BTreeMap;
use iron::prelude::*;
use iron::status;
use handlebars_iron::Template;

pub fn handle_landing_page(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Template::new("landing", ()))))
}

pub fn handle_blog_list_page(_: &mut Request) -> IronResult<Response> {
    use blog::Metadata;

    let posts: Vec<Metadata> = ::blog::read_posts_from_disk()
        .unwrap_or(Vec::new()) // TODO: If empty, display status message instead
        .iter().map(|post| post.metadata.clone())
        .collect();

    let mut template_data = BTreeMap::new();
    template_data.insert(String::from("posts"), posts);

    Ok(Response::with((status::Ok, Template::new("blog_list", template_data))))
}

pub fn handle_blog_post_page(request: &mut Request) -> IronResult<Response> {
    use std::collections::HashMap;
    use hoedown::{ Buffer, Html, Markdown, Render };
    use hoedown::renderer::html::Flags;
    use router::Router;

    // TODO: This should map to BlogPost instead of String
    let paths_to_content: HashMap<String, String> = ::blog::read_posts_from_disk()
        .unwrap_or(Vec::new())
        .iter().map(|post| (post.metadata.get("path").unwrap().clone(), post.body.clone()))
        .collect();

    let ref post_path = request.extensions.get::<Router>().unwrap().find("post").unwrap_or("/");
    let not_found_message = String::from("Blog post not found");
    let content: &str = paths_to_content.get(&String::from(*post_path)).unwrap_or(&not_found_message);

    let post_as_markdown = Markdown::from(Buffer::from(content));
    let mut html_renderer = Html::new(Flags::empty(), 0);
    let post_as_html = html_renderer.render(&post_as_markdown).to_str().expect("Attempting to convert Markdown to HTML").to_string();

    let mut template_data = BTreeMap::new();
    template_data.insert(String::from("post"), post_as_html);
    Ok(Response::with((status::Ok, Template::new("blog_post", template_data))))
}
