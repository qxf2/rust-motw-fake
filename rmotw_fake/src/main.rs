use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

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
    let seed = [
    1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0,
    ];
    let ref mut r = StdRng::from_seed(seed);
    for _ in 0..2 {
        let v: usize = Faker.fake_with_rng(r);
        println!("value from fixed seed {}", v);
    }
    dummy_profile();
}