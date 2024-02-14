/*
Example program showing the the usage of faker module to generate fake data for specific formats and locales
It creates fake data for a social media profile consisting of common fields like id, email, city, phone number.
For the sake of simplicity, this example utilizes these as tuple of values.
However, in a real-world scenario, a struct would typically be used.
It also provides some tests to verify the generated values are as expected
to show how fake data can be used in a real world scenario.
*/

use fake::faker::address::raw::CityName;
use fake::faker::internet::raw::FreeEmail;
use fake::faker::phone_number::raw::CellNumber;
use fake::uuid::UUIDv1;
use fake::{locales::*, Fake};
use uuid::Uuid;

pub fn generate_fake_user_profile() -> (Uuid, String, String, String) {
    // Generate a version 1 UUID which is time-based.
    let user_id: Uuid = UUIDv1.fake();
    // Generate a fake email address with a domain in English.
    let email: String = FreeEmail(EN).fake();
    // Generate a fake city name in French (France locale).
    let city: String = CityName(FR_FR).fake();
    // Generate a fake cell phone number in English.
    let phone_number: String = CellNumber(EN).fake();

    (user_id, email, city, phone_number)
}

fn main() {
    // Generate a fake user profile and print it to the console.
    let (user_id, email, city, phone_number) = generate_fake_user_profile();

    println!(
        "User ID: {}\nEmail: {}\nCity: {}\nPhone Number: {}",
        user_id, email, city, phone_number
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the `generate_fake_user_profile` function to ensure it generates data within expected ranges
    #[test]
    fn test_user_profile_generation() {
        let (user_id, email, city, phone_number) = generate_fake_user_profile();

        assert_eq!(
            user_id.get_version_num(),
            1,
            "User ID should be a UUID version 1"
        );

        assert!(email.contains('@'), "Generated email should contain '@'");
        assert!(
            email.ends_with(".com") || email.ends_with(".net") || email.ends_with(".org"),
            "Generated email should end with a common domain"
        );

        assert!(!city.is_empty(), "Generated city should not be empty");

        assert!(
            !phone_number.is_empty(),
            "Generated phone number should not be empty"
        );
        assert!(
            phone_number.chars().all(|c| c.is_digit(10) || c == '-'),
            "Generated phone number should only contain digits and hyphens"
        );
    }
}
