// Example program to demonstrate the use of collections while generating fake data

use fake::{self, Fake, Faker};
use std::collections::VecDeque;
use fake::Dummy;
use fake::faker::name::en::Name;
use std::rc::Rc;
use crate::fake::faker::company::en::CompanyName;

#[derive(Debug, Dummy)]
pub struct Story {
    #[dummy(faker = "CompanyName()")]
    company: String,
}

fn main() {
    let friends = fake::vec_deque![String as Name(); 5];
    println!("Friends {:?}", friends);

    let mut stories: VecDeque<Rc<Story>> = VecDeque::new();

    // Add new stories to the front of the deque
    for friend in &friends {
        let story: Story = Faker.fake();
        let story = Rc::new(story);
        stories.push_front(story.clone());
        println!("Company of {}: {}", friend, story.company);
    }
    // Remove old stories from the back of the deque if the limit is reached
    let limit = 3;
    while stories.len() > limit {
        stories.pop_back();
    }

    impl std::fmt::Display for Story {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Company: {}", self.company)
        }
    }

    
    println!("Recent updates:");
    for (i, story) in stories.iter().enumerate() {
        println!("{}. {}", i + 1, story);
    }
    
}