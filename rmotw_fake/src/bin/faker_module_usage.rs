/*
Example program showing the the usage of 'faker' module to generate fake data 
for specific formats and locales.
It simulates a social media platform that offers personalized content 
recommendation to its users based on their profile.
*/

use fake::faker::address::raw::CityName;
use fake::faker::internet::raw::FreeEmail;
use fake::faker::phone_number::raw::CellNumber;
use fake::uuid::UUIDv1;
use fake::{locales::{EN, FR_FR}, Fake};
use uuid::Uuid;

// UserProfile struct represents a profile of a user
#[derive(Debug)]
pub struct UserProfile {
    pub user_id: Uuid,  
    pub email: String,  
    pub city: String,  
    pub phone_number: String,  
    pub city_category: String,  
    pub recommended_content: String, 
}

// Function to generate a fake user profile
pub fn generate_fake_user_profile() -> UserProfile {
    let user_id: Uuid = UUIDv1.fake(); 
    let email: String = FreeEmail(EN).fake();  
    let city: String = CityName(FR_FR).fake();  
    let phone_number: String = CellNumber(EN).fake();  
    // Categorize the city and recommend content based on the category
    let (city_category, recommended_content) = categorize_city_and_recommend_content(&city);

    // Return a UserProfile with the generated data
    UserProfile {
        user_id,
        email,
        city,
        phone_number,
        city_category,
        recommended_content,
    }
}

// Function to categorize a city and recommend content based on the category
pub fn categorize_city_and_recommend_content(city: &str) -> (String, String) {
    let city_category = if city.contains("Paris") || city.contains("Lyon") || city.contains("Marseille") {
        "major".to_string()
    } else {
        "smaller".to_string()
    };

    // Recommend content based on the city category
    let recommended_content = match city_category.as_str() {
        "major" => "global news and events".to_string(),
        "smaller" => "local news and events".to_string(),
        _ => "no content available".to_string(),
    };

    // Return the city category and the recommended content
    (city_category, recommended_content)
}

fn main() {
    // Generate a fake user profile.
    let user_profile = generate_fake_user_profile();

    // Print the information of the user profile
    println!(
        "User ID: {}\nEmail: {}\nCity: {}\nPhone Number: {}\nCity Category: {}\nRecommended Content: {}",
        user_profile.user_id,
        user_profile.email,
        user_profile.city,
        user_profile.phone_number,
        user_profile.city_category,
        user_profile.recommended_content
    );
}
