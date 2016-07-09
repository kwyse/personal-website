//! Provides routes for main application

use std::collections::BTreeMap;
use blog::BlogPost;
use iron::prelude::*;
use iron::status;
use handlebars_iron::Template;
use rustc_serialize::json::ToJson;

pub fn handle_landing_page(_: &mut Request) -> IronResult<Response> {
    handle_with_template("landing", ())
}

pub fn handle_about_page(_: &mut Request) -> IronResult<Response> {
    handle_with_template("about", ())
}

pub fn handle_blog_list_page(_: &mut Request) -> IronResult<Response> {
    use blog::Metadata;

    let posts: Vec<Metadata> = get_posts()
        .iter().map(|post| post.metadata.clone())
        .collect();

    let mut template_data = BTreeMap::new();
    template_data.insert(String::from("posts"), posts);
    handle_with_template("blog_list", template_data)
}

pub fn handle_blog_post_page(request: &mut Request) -> IronResult<Response> {
    use std::collections::HashMap;
    use router::Router;

    let posts = get_posts();
    let paths_index = posts.iter().map(|post| {
        // TODO: Ensure path is always available instead of constructing
        (post.metadata.get("path").unwrap_or(&construct_path(post)).clone(), post.clone())
    }).collect::<HashMap<_, _>>();

    let router_extension = request.extensions.get::<Router>().unwrap(); // TODO: Handle this
    let post_path = router_extension.find("post").unwrap_or("/");
    let post = paths_index.get(&String::from(post_path)).unwrap(); // TODO: Handle post not found

    let mut template_data = BTreeMap::new();
    template_data.insert(String::from("body"), post.get_body_as_html());
    template_data.insert(String::from("title"), post.metadata.get("title").unwrap_or(&String::from("No title")).clone());
    template_data.insert(String::from("date"), post.metadata.get("date").unwrap_or(&String::from("No date")).clone());
    handle_with_template("blog_post", template_data)
}

fn handle_with_template<T: ToJson>(name: &str, data: T) -> IronResult<Response> {
    Ok(Response::with(
        (status::Ok, Template::new(name, data))
    ))
}

// TODO: Do this when constructing BlogPost
fn construct_path(post: &BlogPost) -> String {
    use std::hash::{ Hash, Hasher, SipHasher };

    let mut hasher = SipHasher::new();
    post.body.hash(&mut hasher);
    format!("{:16}", hasher.finish())
}


fn get_posts() -> Vec<BlogPost> {
    use blog::read_posts_from_disk;

    read_posts_from_disk().unwrap_or(Vec::new())
}
