// Example of using faker module to generate fake values for specific formats, can generate fake details with more details and options.
// It has different helpers in different categories, such as name, address, lorem, etc. The faker module is structured in a way that each category has a raw module and several localized modules. The raw module provides a generic helper that can create fake data in any locale that you specify. The localized modules provide convenient helpers that can create fake data in a specific locale that is already defined.

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

