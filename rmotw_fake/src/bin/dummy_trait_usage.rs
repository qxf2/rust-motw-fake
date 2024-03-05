/*
Example program demonstrates the usage of the 'Dummy' trait to generate fake data.
It defines a social media structure with posts and comments and populates these with
realistic-looking test data.
*/

use fake::faker::boolean::en::Boolean;
use fake::faker::name::en::Name;
use fake::Dummy;
use fake::Faker;

#[derive(Debug, Dummy)]
pub enum PostStatus {
    Published,
    Draft,
}

// SocialMediaPost is a struct representing a post on social media. 
#[derive(Debug, Dummy)]
pub struct SocialMediaPost {
    #[dummy(faker = "1000..")]
    pub post_id: usize,

    #[dummy(faker = "Name()")]
    pub author: String,

    #[dummy(faker = "(Faker, 3..5)")]
    pub comments: Vec<Comment>,

    #[dummy(faker = "Boolean(80)")]
    pub is_public: bool,

    pub status: PostStatus,
}

#[derive(Debug, Dummy)]
pub struct Comment {
    #[dummy(faker = "1..100")]
    pub comment_id: usize,

    pub text: String,

    #[dummy(faker = "Name()")]
    pub commenter: String,
}

// function to generate fake posts by accepting number of posts to be generated
pub fn generate_fake_post(num_posts: usize) -> Vec<SocialMediaPost> {
    fake::vec![SocialMediaPost; num_posts]
}

pub fn main() {
    let posts = generate_fake_post(2);
    println!("{:#?}", posts);
}