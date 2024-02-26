/*
Example program to demonstrate the use of Rust collections and the `fake` crate to generate fake data.
It uses an example to create a double ended queue to manage a list of friends and stories associated with them.
*/

use fake::{self, Fake, Faker};
use std::collections::VecDeque;
use fake::Dummy;
use fake::faker::name::en::Name;
use std::rc::Rc;
use crate::fake::faker::lorem::en::Sentence;

//Define a `Story` struct with a `comment` field that will be filled with fake comments
#[derive(Debug, Dummy)]
pub struct Story {
    #[dummy(faker = "Sentence(5..10)")]
    comment: String,
}

fn main() {
    // Generate a VecDeque of fake friend names
    let friends = fake::vec_deque![String as Name(); 5];
    
    println!("Generated friends list:");
    // Print each friend's name
    for friend in &friends {
        println!(" - {}", friend);
    }

    // Create an empty VecDeque to hold `Story` objects wrapped in `Rc` (reference-counted pointers)
    let mut stories: VecDeque<Rc<Story>> = VecDeque::new();

    // Add new stories to the front of the deque for each friend
    for friend in &friends {
        let story: Story = Faker.fake();
        let story = Rc::new(story);
        stories.push_front(story.clone());
        // Print the comment name associated with each friend's story
        println!("Comment for {}: {}", friend, story.comment);
    }

    // Limit the number of stories to keep in the deque
    let limit = 3;
    // Remove the oldest stories once the limit is exceeded
    while stories.len() > limit {
        stories.pop_back();
    }

    // Implement the Display trait for `Story` to allow for custom printing
    impl std::fmt::Display for Story {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // Print the comment name of the story
            write!(f, "comment: {}", self.comment)
        }
    }
    
    println!("Recent updates:");
    // Enumerate through the stories and print them with an index
    for (i, story) in stories.iter().enumerate() {
        println!("{}. {}", i + 1, story);
    }
}


