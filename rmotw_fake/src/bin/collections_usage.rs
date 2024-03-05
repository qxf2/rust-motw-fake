/*
Example program to demonstrate the use of Rust collections to generate fake data.
It uses an example to create a double ended queue to manage 
a list of authors and posts associated with them.
*/

use fake::{self, Dummy};
use std::collections::VecDeque;
use fake::faker::lorem::en::Sentence;
use fake::faker::name::en::Name;

// Define a 'Post' struct with content and author fields that will be filled with fake content
#[derive(Debug, Dummy)]
pub struct Post {
    #[dummy(faker = "Sentence(5..10)")]
    content: String,
    #[dummy(faker = "Name()")]
    author: String,
}

// Function to filter out posts shorter than certain length
fn filter_posts(posts: VecDeque<Post>, min_length: usize) -> VecDeque<Post> {
    posts.into_iter().filter(|post| post.content.len() >= min_length).collect()
}

fn main() {
    // Generate a VecDeque of fake 'Post' objects
    let posts: VecDeque<Post> = fake::vec_deque![Post; 5];

    println!("Generated posts:");
    for post in &posts {
        println!(" - Content: {}\n   Author: {}", post.content, post.author);
    }

    // Filter out posts that are shorter than a certain length
    let min_length = 30; 
    let filtered_posts = filter_posts(posts, min_length);

    println!("\nPosts after filtering:");
    for post in &filtered_posts {
        println!(" - Content: {}\n   Author: {}", post.content, post.author);
    }
}


