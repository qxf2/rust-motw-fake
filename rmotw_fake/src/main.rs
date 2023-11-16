use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::prelude::*;

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

    if rand::random() {
        println!("Random boolean is true");
        // Try printing a random unicode code point (probably a bad idea)!
        println!("Random char: {}", rand::random::<char>());
    } else {
        println!("Random boolean is false");
    }

    // Random float between 0 and 1
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("Random float: {}", y);

    // Shuffled vector of integers
    let mut nums: Vec<i32> = (1..5).collect();
    nums.shuffle(&mut rng);
    println!("Shuffled vector: {:?}", nums);
    
}
