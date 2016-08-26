//! Provides routes for main application

use iron::prelude::*;
use iron::status;
use handlebars_iron::Template;
use serde::Serialize;

use ::db::read_posts;

pub fn handle_landing_page(_: &mut Request) -> IronResult<Response> {
    handle_with_template("landing", ())
}

pub fn handle_about_page(_: &mut Request) -> IronResult<Response> {
    handle_with_template("about", ())
}

pub fn handle_blog_list_page(_: &mut Request) -> IronResult<Response> {
    let posts = read_posts();

    if posts.is_empty() {
        handle_with_template("blog_list_noposts", "No blog posts found. Check back soon!")
    } else {
        handle_with_template("blog_list", posts)
    }
}

pub fn handle_blog_post_page(request: &mut Request) -> IronResult<Response> {
    use std::collections::HashMap;
    use router::Router;

    let mut posts = read_posts();
    let mut paths_index = posts.iter_mut().map(|post| (post.url.clone(), post)).collect::<HashMap<_, _>>();

    let router_extension = request.extensions.get::<Router>().unwrap(); // TODO: Handle this
    let post_path = router_extension.find("post").unwrap_or("/");
    let post: &mut ::db::models::BlogPost = paths_index.get_mut(&String::from(post_path)).unwrap(); // TODO: Handle post not found
    post.body = markdown_to_html(&post.body);

    handle_with_template("blog_post", post)
}

fn handle_with_template<T: Serialize>(name: &str, data: T) -> IronResult<Response> {
    Ok(Response::with(
        (status::Ok, Template::new(name, data))
    ))
}

fn markdown_to_html(markdown: &str) -> String {
    use hoedown::{ Html, Markdown, Render };
    use hoedown::{ TABLES, FENCED_CODE, AUTOLINK, STRIKETHROUGH, NO_INTRA_EMPHASIS };
    use hoedown::renderer::html::Flags;

    let mut renderer = Html::new(Flags::empty(), 0);
    let extensions = TABLES | FENCED_CODE | AUTOLINK | STRIKETHROUGH | NO_INTRA_EMPHASIS;
    renderer.render(&Markdown::new(markdown).extensions(extensions))
        .to_str().unwrap_or("Unable to render Markdown body")
        .to_string()
}
