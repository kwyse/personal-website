//! Abstraction for database communication

use diesel::prelude::*;
use diesel::pg::PgConnection;

use self::models::*;

pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn read_posts() -> Vec<BlogPost> {
    use std::error::Error;
    use db::schema::blogposts::dsl::*;

    let conn = establish_connection();
    blogposts.load::<BlogPost>(&conn).unwrap_or_else(|err| {
        warn!("Error loading blog posts from database - returning empty Vec");
        error!("{}", err.description());
        Vec::new()
    })
}

pub fn create_post(post: NewBlogPost) -> BlogPost {
    use db::schema::blogposts;
    // use db::schema::blogposts::dsl::*;

    let conn = establish_connection();
    ::diesel::insert(&post).into(blogposts::table).get_result(&conn).expect("Error adding new post")
}

pub mod models {
    use super::schema::blogposts;
    use chrono::NaiveDate;
    
    #[derive(Queryable, Serialize)]
    pub struct BlogPost {
        pub id: i32,
        pub title: String,
        pub created: NaiveDate,
        pub published: bool,
        pub url: String,
        pub summary: String,
        pub body: String,
    }

    #[insertable_into(blogposts)]
    pub struct NewBlogPost {
        pub title: String,
        pub created: NaiveDate,
        pub url: String,
        pub summary: String,
        pub body: String,
    }
}

pub mod schema {
    infer_schema!(dotenv!("DATABASE_URL"));
}
