//This example shows how to use Dummy Trait for generating fake data. This is useful for data structures

use fake::faker::boolean::en::*;
use fake::faker::name::en::*;
use fake::Dummy;
use fake::{Fake, Faker};

#[derive(Debug, Dummy)]
pub enum PostStatus {
    Published,
    Draft,
}

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

pub fn generate_fake_post() {
    let post: SocialMediaPost = Faker.fake();
    println!("{:#?}", post);  
    
}

pub fn main() {
    generate_fake_post();
}