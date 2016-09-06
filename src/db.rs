//! Abstraction for database communication

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::error::Error;

use self::models::*;

pub fn create_post(conn: &PgConnection, post: NewBlogPost) -> BlogPost {
    use db::schema::blogposts;

    ::diesel::insert(&post).into(blogposts::table)
        .get_result(&*conn)
        .expect("Error adding new post")
}

pub fn read_posts(conn: &PgConnection) -> Vec<BlogPost> {
    read_tagged_posts(conn, Vec::new())
}

pub fn read_tagged_posts(conn: &PgConnection, target_tags: Vec<String>) -> Vec<BlogPost> {
    use db::schema::blogposts::dsl::*;

    if target_tags.is_empty() {
        blogposts.filter(published.eq(true))
            .order(created.desc())
            .load::<BlogPost>(&*conn)
            .unwrap_or_else(handle_load_error)
    } else {
        blogposts.filter(published.eq(true))
            .filter(tags.overlaps_with(target_tags))
            .order(created.desc())
            .load::<BlogPost>(&*conn)
            .unwrap_or_else(handle_load_error)
    }
}

pub fn publish_post(conn: &PgConnection, target_id: i32) -> BlogPost {
    use db::schema::blogposts::dsl::*;

    ::diesel::update(blogposts.filter(id.eq(target_id)))
        .set(published.eq(true))
        .get_result(&*conn)
        .expect("Error publishing post")
}

pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn handle_load_error<E: Error>(err: E) -> Vec<BlogPost> {
    warn!("Error loading blog posts from database - returning empty Vec");
    error!("{}", err.description());
    Vec::new()
}

pub mod models {
    use super::schema::blogposts;
    use chrono::NaiveDate;
    
    #[derive(Debug, PartialEq, Queryable, Serialize)]
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
    fn test_create_post() {
        use chrono::NaiveDate;
        use diesel::LoadDsl;
        use super::models::*;
        use super::schema::blogposts;

        let conn = get_test_connection();
        let mut num_posts = conn.execute("SELECT * FROM blogposts").unwrap();
        assert_eq!(0, num_posts);

        let new_post = NewBlogPost {
            title: "New Post Title".to_string(),
            created: NaiveDate::from_ymd(2016, 8, 5),
            url: "new-post".to_string(),
            summary: "A new summary".to_string(),
            body: "A new body".to_string(),
            tags: vec!["new".to_string(), "post".to_string()],
        };

        create_post(&conn, new_post);
        num_posts = conn.execute("SELECT * FROM blogposts").unwrap();
        assert_eq!(1, num_posts);

        let expected_post = BlogPost {
            id: 1,
            title: "New Post Title".to_string(),
            created: NaiveDate::from_ymd(2016, 8, 5),
            published: false,
            url: "new-post".to_string(),
            summary: "A new summary".to_string(),
            body: "A new body".to_string(),
            tags: vec!["new".to_string(), "post".to_string()],
        };

        let actual_post = blogposts::table.get_result::<BlogPost>(&conn).unwrap();
        assert_eq!(expected_post, actual_post);
    }

    #[test]
    fn test_read_posts() {
        use chrono::NaiveDate;

        let conn = get_test_connection();
        let mut posts = read_posts(&conn);

        assert!(posts.is_empty());

        conn.execute("INSERT INTO blogposts VALUES
                      (1, 'Test Post', '20160722', 'f', 'test-post', 'A test post', 'A body', '{test}');"
        ).unwrap();
        posts = read_posts(&conn);
        assert!(posts.is_empty());

        let _ = conn.execute("UPDATE blogposts SET published = 't' WHERE id = 1;");
        posts = read_posts(&conn);
        assert_eq!(1, posts.len());

        conn.execute("INSERT INTO blogposts VALUES
                      (2, 'Test Post', '20160822', 't', 'test-post', 'A test post', 'A body', '{test}');"
        ).unwrap();
        posts = read_posts(&conn);
        assert_eq!(2, posts.len());
        assert_eq!(NaiveDate::from_ymd(2016, 8, 22), posts[0].created);
        assert_eq!(NaiveDate::from_ymd(2016, 7, 22), posts[1].created);
    }

    #[test]
    fn test_read_tagged_posts() {
        let conn = get_test_connection();
        conn.execute("INSERT INTO blogposts VALUES
                      (1, 'Test Post', '20160722', 't', 'test-post', 'A test post', 'A body', '{test}');"
        ).unwrap();
        let mut posts = read_tagged_posts(&conn, vec!["nottest".to_string()]);
        assert!(posts.is_empty());

        let _ = conn.execute("UPDATE blogposts SET tags = '{test, post}' WHERE id = 1;");
        posts = read_tagged_posts(&conn, vec!["test".to_string()]);
        assert_eq!(1, posts.len());

        conn.execute("INSERT INTO blogposts VALUES
                      (2, 'Test Post', '20160822', 't', 'test-post', 'A test post', 'A body', '{post}');"
        ).unwrap();
        posts = read_tagged_posts(&conn, vec!["post".to_string()]);
        assert_eq!(2, posts.len());
    }

    #[test]
    fn test_publish_post() {
        use diesel::{ExpressionMethods, FilterDsl, LoadDsl};
        use super::models::BlogPost;
        use super::schema::blogposts::dsl::*;

        let conn = get_test_connection();
        conn.execute("INSERT INTO blogposts VALUES
                      (1, 'Test Post', '20160722', 'f', 'test-post', 'A test post', 'A body', '{test}');"
        ).unwrap();
        conn.execute("INSERT INTO blogposts VALUES
                      (2, 'Test Post', '20160822', 'f', 'test-post', 'A test post', 'A body', '{post}');"
        ).unwrap();

        publish_post(&conn, 1);

        let post1 = blogposts.filter(id.eq(1)).get_result::<BlogPost>(&conn).unwrap();
        let post2 = blogposts.filter(id.eq(2)).get_result::<BlogPost>(&conn).unwrap();
        assert_eq!(true, post1.published);
        assert_eq!(false, post2.published);
    }
}
