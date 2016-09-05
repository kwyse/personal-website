//! Abstraction for database communication

use diesel::prelude::*;
use diesel::pg::PgConnection;

use self::models::*;

pub fn create_post(post: NewBlogPost) -> BlogPost {
    use db::schema::blogposts;

    let conn = establish_connection();
    ::diesel::insert(&post).into(blogposts::table)
        .get_result(&conn)
        .expect("Error adding new post")
}

pub fn read_posts(conn: &PgConnection) -> Vec<BlogPost> {
    use std::error::Error;
    use db::schema::blogposts::dsl::*;

    blogposts.filter(published.eq(true))
        .order(created.desc())
        .load::<BlogPost>(&*conn).unwrap_or_else(|err| {
            warn!("Error loading blog posts from database - returning empty Vec");
            error!("{}", err.description());
            Vec::new()
        })
}

pub fn read_tagged_posts(target_tags: Vec<String>) -> Vec<BlogPost> {
    use std::error::Error;
    use db::schema::blogposts::dsl::*;

    let conn = establish_connection();
    blogposts.filter(published.eq(true))
        .filter(tags.overlaps_with(target_tags))
        .order(created.desc())
        .load::<BlogPost>(&conn).unwrap_or_else(|err| {
            warn!("Error loading blog posts from database - returning empty Vec");
            error!("{}", err.description());
            Vec::new()
        })
}

pub fn publish_post(target_id: i32) -> BlogPost {
    use db::schema::blogposts::dsl::*;

    let conn = establish_connection();
    ::diesel::update(blogposts.filter(id.eq(target_id)))
        .set(published.eq(true))
        .get_result(&conn)
        .expect("Error publishing post")
}

pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
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
        pub tags: Vec<String>,
    }

    #[insertable_into(blogposts)]
    pub struct NewBlogPost {
        pub title: String,
        pub created: NaiveDate,
        pub url: String,
        pub summary: String,
        pub body: String,
        pub tags: Vec<String>,
    }
}

pub mod schema {
    infer_schema!(dotenv!("DATABASE_URL"));
}

#[cfg(test)]
mod tests {
    use diesel::pg::PgConnection;
    use diesel::Connection;

    use super::*;

    fn get_test_connection() -> PgConnection {
        use std::io;
        use diesel::migrations;

        let conn_url = "postgres://localhost/test";
        let conn = PgConnection::establish(&conn_url).unwrap();
        conn.begin_test_transaction().unwrap();

        let migrations_dir = migrations::find_migrations_directory().unwrap();
        migrations::run_pending_migrations_in_directory(&conn, &migrations_dir, &mut io::sink()).unwrap();

        conn
    }

    #[test]
    fn test_read_posts() {
        use chrono::NaiveDate;
        let conn = get_test_connection();
        let mut posts = read_posts(&conn);

        assert!(posts.len() == 0);

        conn.execute("INSERT INTO blogposts VALUES (1, 'Test Post', '20160722', 'f', 'test-post', 'A test post', 'A body', '{test}');").unwrap();
        posts = read_posts(&conn);
        assert!(posts.len() == 0);

        let _ = conn.execute("UPDATE blogposts SET published = 't' WHERE id = 1;");
        posts = read_posts(&conn);
        assert!(posts.len() == 1);

        conn.execute("INSERT INTO blogposts VALUES (2, 'Test Post', '20160822', 't', 'test-post', 'A test post', 'A body', '{test}');").unwrap();
        posts = read_posts(&conn);
        assert!(posts.len() == 2);
        assert!(posts[0].created == NaiveDate::from_ymd(2016, 8, 22));
        assert!(posts[1].created == NaiveDate::from_ymd(2016, 7, 22));
    }
}
