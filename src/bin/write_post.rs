extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use std::io::{stdin, Read};

#[cfg(not(windows))]
const EOF: &'static str = "<CTRL+D>";
#[cfg(windows)]
const EOF: &'static str = "<CTRL+Z>";

fn main() {
    let connection = establish_connection();

    println!("Title?:");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("Ok. What is the body for \"{}\"?\nPress {} when finished.", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, title, &body);
    println!("\nSaved draft of \"{}\" with id {}", title, post.id);
}
