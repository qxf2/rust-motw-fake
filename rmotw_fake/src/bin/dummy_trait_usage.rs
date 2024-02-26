/*
Example program demonstrates the usage of the `fake` crate to generate
fake data for testing purposes. It defines a social media structure with posts
and comments and uses the `Dummy` trait to populate these structures with
realistic-looking test data.
*/

use fake::faker::boolean::en::*;
use fake::faker::name::en::*;
use fake::Dummy;
use fake::{Fake, Faker};

// `SocialMediaPost` is an enum representing the possible states of a social media post.
#[derive(Debug, Dummy)]
pub enum PostStatus {
    Published,
    Draft,
}

// `SocialMediaPost` is a struct representing a post on social media.
#[derive(Debug, Dummy)]
pub struct SocialMediaPost {
    // Each post has a unique ID, which is a number starting from 1000.
    #[dummy(faker = "1000..")]
    pub post_id: usize,

    // The author field is populated with a fake name.
    #[dummy(faker = "Name()")]
    pub author: String,

    // A post has 3 to 5 comments, each represented by a `Comment` struct.
    #[dummy(faker = "(Faker, 3..5)")]
    pub comments: Vec<Comment>,

    // The `is_public` field is a boolean that's `true` for approximately 80% of the generated posts.
    #[dummy(faker = "Boolean(80)")]
    pub is_public: bool,

    // The status of the post, which can either be `Published` or `Draft`.
    pub status: PostStatus,
}

// `Comment` is a struct representing a comment on a social media post.
#[derive(Debug, Dummy)]
pub struct Comment {
    #[dummy(faker = "1..100")]
    pub comment_id: usize,

    pub text: String,

    #[dummy(faker = "Name()")]
    pub commenter: String,
}

// function that generates a single fake social media post and prints it to the console.
// It also demonstrates generating multiple posts using a vector.
pub fn generate_fake_post() {
    let post: SocialMediaPost = Faker.fake();
    println!("Generating fake social media post...");
    println!("{:#?}", post);

    println!("Generating multiple fake social media posts...");
    let posts = fake::vec![SocialMediaPost; 3];
    println!("{:#?}", posts);
}

pub fn main() {
    generate_fake_post();
}

// Test the `generate_fake_post` function to ensure it generates data within expected ranges

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_media_post_generation() {
        // Generate a single fake social media post and perform validation on the fields
        let fake_post: SocialMediaPost = Faker.fake();
        assert!(fake_post.post_id >= 1000);
        assert!(!fake_post.author.is_empty());
        assert!(fake_post.comments.len() >= 3 && fake_post.comments.len() <= 5);

        // Check that 80% of the generated posts are public
        let mut public_count = 0;
        let total_posts = 100;
        for _ in 0..total_posts {
            let temp_post: SocialMediaPost = Faker.fake();
            if temp_post.is_public {
                public_count += 1;
            }
        }
        assert!(public_count >= 80);
    }

    // Example of using fake data in a real-world scenario:
    // Simulate a scenario where a new social media platform is being tested for load
    // and data consistency by generating a large number of fake posts and comments.
    #[test]
    fn simulate_social_media_platform() {
        let number_of_posts = 10;
        let mut posts = Vec::with_capacity(number_of_posts);

        for _ in 0..number_of_posts {
            let post: SocialMediaPost = Faker.fake();
            posts.push(post);
        }

        // Perform the test to ensure all posts meet certain criteria
        for post in posts {
            assert!(post.post_id >= 1000);
            assert!(!post.author.is_empty());
            assert!(post.comments.len() >= 3 && post.comments.len() <= 5);
        }
    }
}
