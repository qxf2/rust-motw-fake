/* 
Example program showing the basic usage of generating fake data using Fake Trait.
It generates fake data related to a social media platform.
For the sake of simplicity, this example utilizes tuple of values.
However, in a real-world scenario, a struct would typically be used. 
 */

use fake::Fake;

fn generate_fake_social_media_data() -> (u32, u16, String, String) {
    let likes_count = (1..1000).fake::<u32>();
    let shares_count = (1..500).fake::<u16>();
    let post_text = (20..50).fake::<String>();
    let caption_text = (5..15).fake::<String>();

    (likes_count, shares_count, post_text, caption_text)
}

// An example function that categorizes a post based on the number of likes it has. 
fn categorize_post(likes_count: u32) -> String {
    if likes_count >= 800 {
        "popular".to_string()
    } else if likes_count >= 500 {
        "average".to_string()
    } else {
        "normal".to_string()
    }
}

// Generate fake social media data and categorize the post based on the number of likes. 
fn main() {
    let (likes_count, shares_count, post_text, caption_text) = generate_fake_social_media_data();
    let post_category = categorize_post(likes_count);
    println!("Likes: {}\nShares: {}\nPost: {:?}\nCaption: {:?}\nCategory: {:?}",
    likes_count, shares_count, post_text, caption_text, post_category);
}
