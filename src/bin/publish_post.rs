extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::diesel_demo::models::*;
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};

    let id: i32 = args().nth(1)
        .expect("publish_post requires the post id as the first argument.")
        .parse().expect("Invalid Id");

    let connection = establish_connection();

    let post: Post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result(&connection)
        .expect(&format!("Unable to find post with id {}", id));
    println!("Published post \"{}\"", post.title);
}
