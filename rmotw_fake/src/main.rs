use fake::{Dummy, Faker, Fake};
use fake::faker::name::en::*;
use fake::faker::address::en::StreetName;
use fake::faker::phone_number::en::*;
use fake::faker::internet::en::SafeEmail;
use fake::faker::company::en::CompanyName;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Debug, Dummy)]
pub struct Profile {
    #[dummy(faker = "Name()")]
    pub name: String,

    #[dummy(faker = "18..80")]
    pub age: usize,

    #[dummy(faker = "StreetName()")]
    pub address: String,

    #[dummy(faker = "PhoneNumber()")]
    pub phone_number: String,

    #[dummy(faker = "SafeEmail()")]
    pub email: String,

    #[dummy(faker = "CompanyName()")]
    pub company: String,
}

fn dummy_profile(rng: &mut StdRng) {
    let profile: Profile = Faker.fake_with_rng(rng);
    println!("{:?}", profile);
}

fn main() {
    let seed = [
        0, 0, 0, 0, 0, 0, 0, 0, 3, 1, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let mut rng = StdRng::from_seed(seed);

    for _ in 0..2 {
        dummy_profile(&mut rng);
    }
}
