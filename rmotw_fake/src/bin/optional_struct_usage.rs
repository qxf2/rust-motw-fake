//Example program that shows how to use optional struct while generating fake data

use fake::{Dummy, Fake, Faker, Opt, Optional};
use fake::faker::name::en::*;

#[derive(Debug, Dummy)]
pub struct Profile {
    #[dummy(faker = "Name()")]
    pub name: String,   

    #[dummy(faker = "Opt(0..80, 50)", from = "Optional<u64>")]
    pub age: Option<u64>,

    #[dummy(faker = "Opt(0..200, 100)", from = "Optional<u64>")]
    pub following: Option<u64>,

    #[dummy(expr = "Some((0..200).fake())")]
    pub followers: Option<u64>,

    #[dummy(faker = "0..200")]
    pub posts: Option<Option<u64>>,
}

pub fn print_profile(profile: Profile) {
    println!("Name: {}", profile.name);
    match profile.age {
        Some(age) => println!("Age: {}", age),
        None => println!("Age: not provided"),
    }
    match profile.followers {
        Some(followers) => println!("Followers: {}", followers),
        None => println!("Followers: not provided"),
    }
    match profile.following {
        Some(following) => println!("Following: {}", following),
        None => println!("Following: not provided"),
    }
}

fn main() {
    let profile: Profile = Faker.fake();
    println!("{:?}", profile);
    print_profile(profile)
}