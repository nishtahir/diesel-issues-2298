#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

use dotenv::dotenv;

use diesel::prelude::*;
use schema::{authors, category, posts};
use std::env;

#[derive(Queryable, Identifiable, Debug, Associations)]
pub struct Author {
    pub id: i32,
    pub first_name: String,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[table_name = "category"]
pub struct Category {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(Category)]
#[belongs_to(Author)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub category_id: i32,
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let author = authors::table.filter(authors::dsl::id.eq(1)).first::<Author>(&connection).unwrap();

    let categories = category::table
        // commenting out this line resolves the issue
        .filter(category::dsl::id.lt(2))
        .load::<Category>(&connection)
        .unwrap();
        
    Post::belonging_to(&author)
        .get_results::<Post>(&connection)
        .unwrap()
        .grouped_by(&categories);
}
