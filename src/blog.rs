//! Abstraction for a blog post
//!
//! Includes utility functions to read posts from disk

use std::collections::HashMap;
use std::io::Result;
use std::path::Path;

pub type Metadata = HashMap<String, String>;

#[derive(Debug)]
pub struct BlogPost {
    pub body: String,
    pub metadata: Metadata,
}

impl BlogPost {
    fn new() -> Self {
        BlogPost {
            body: String::new(),
            metadata: Metadata::new(),
        }
    }
}

impl BlogPost {
    pub fn from_file<P: AsRef<Path>>(file_path: P) -> Result<BlogPost> {
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
                    post.metadata.insert(String::from(first.to_lowercase()), second);
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

    pub fn get_body_as_html(&self) -> String {
        use hoedown::{ Html, Markdown, Render };
        use hoedown::renderer::html::Flags;
        use hoedown::{ TABLES, FENCED_CODE, AUTOLINK, STRIKETHROUGH, NO_INTRA_EMPHASIS };

        let mut renderer = Html::new(Flags::empty(), 0);
        let extensions = TABLES | FENCED_CODE | AUTOLINK | STRIKETHROUGH | NO_INTRA_EMPHASIS;
        renderer.render(&Markdown::new(&self.body).extensions(extensions))
            .to_str().unwrap_or("Unable to render Markdown body")
            .to_string()
    }
}

pub fn read_posts_from_disk() -> Result<Vec<BlogPost>> {
    use std::fs::read_dir;

    let dir_entries = try!(read_dir("posts/"));
    let mut posts: Vec<BlogPost> = Vec::new();
    for entry in dir_entries {
        let file_path = try!(entry).path();
        posts.push(try!(BlogPost::from_file(file_path)));
    }

    Ok(posts)
}
