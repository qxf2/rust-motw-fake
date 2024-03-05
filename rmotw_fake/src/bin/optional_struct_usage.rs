//Example program that shows how to use optional struct while generating fake data

use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::Name;

#[derive(Debug, Dummy)]
pub struct Profile {
    #[dummy(faker = "Name()")]
    pub name: String,   

    #[dummy(faker = "0..=80")]
    pub age: Option<u64>,

    #[dummy(faker = "0..=200")]
    pub following: Option<u64>,

    #[dummy(expr = "Some((0..=200).fake())")]
    pub followers: Option<u64>,

    #[dummy(faker = "0..=200")]
    pub posts: Option<Option<u64>>,
}

pub fn print_profile(profile: Profile) {
    println!("Profile {{\nname: \"{}\",\nage: {:?},\nfollowing: {:?},\nfollowers: {:?},\nposts: {:?}\n}}",
    profile.name, profile.age, profile.following, profile.followers, profile.posts);
}

fn main() {
    let profile: Profile = Faker.fake();
    print_profile(profile)
}