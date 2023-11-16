use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::*;
use fake::locales::*;

#[derive(Debug, Dummy)]
pub struct Profile {
    #[dummy(faker = "Name()")]
    pub name: String,

    #[dummy(faker = "18..80")]
    pub age: usize,
}

fn dummy_profile() {
    let profile: Profile = Faker.fake();
    println!("{:?}", profile);    
}

fn main() {
    dummy_profile();
}