// Example program showing the usage of faker module to generate fake data for specific formats and locales

use fake::{Fake};
use fake::locales::{EN, FR_FR}; 
use fake::faker::phone_number::raw::*;
use fake::faker::address::raw::*;
use fake::faker::internet::raw::FreeEmail;
use fake::uuid::*;
use uuid::Uuid;

pub fn generate_fake_user_profile() {

    let user_id: Uuid = UUIDv1.fake();
    let email: String = FreeEmail(FR_FR).fake();
    let city: String = CityName(EN).fake();
    let phone_number: String = PhoneNumber(EN).fake();

    println!(
        "User ID: {}\nEmail: {}\nCity: {}\nPhone Number: {}",
        user_id, email, city, phone_number
    );
}

fn main() {
    generate_fake_user_profile();
}

