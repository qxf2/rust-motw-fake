/*
Example program showing the basic usage of generating fake data using Fake Trait.
It creates fake data for a social media app post consisting of likes, shares, post and caption.
For the sake of simplicity, this example utilizes these as tuple of values.
However, in a real-world scenario, a struct would typically be used.
It also provides test to verify the generated values are as expected 
to show how fake data can be used in a real world scenario.
*/

use fake::Fake;

fn generate_fake_social_media_data() -> (u32, u16, String, String) {
    // Generate a random number of likes between 1 and 1000.
    let likes_count = (1..1000).fake::<u32>();
    // Generate a random number of shares between 1 and 100.
    let shares_count = (1..500).fake::<u16>();
    // Generate a random string for post text with a length between 20 and 50 characters.
    let post_text = (20..50).fake::<String>();
    // Generate a random string for a caption with a length between 5 and 15 characters.
    let caption_text = (5..15).fake::<String>();

    (likes_count, shares_count, post_text, caption_text)
}

fn main() {
    // Call `generate_fake_social_media_data` to get fake social media data.
    let (likes_count, shares_count, post_text, caption_text) = generate_fake_social_media_data();
    println!("Likes: {}\nShares: {}\nPost: {:?}\nCaption: {:?}",
    likes_count, shares_count, post_text, caption_text);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the `generate_fake_social_media_data` function to ensure it generates data within expected ranges
    #[test]
    fn test_generate_fake_social_media_data() {
        let (likes_count, shares_count, post_text, caption_text) =
            generate_fake_social_media_data();

        assert!(
            likes_count >= 1 && likes_count < 1000,
            "Likes should be between 1 and 999"
        );

        assert!(
            shares_count >= 1 && shares_count < 500,
            "Shares should be between 1 and 499"
        );

        assert!(
            post_text.len() >= 20 && post_text.len() < 50,
            "Post post_text length should be between 20 and 50"
        );

        assert!(!caption_text.is_empty(), "Comment should not be empty");
        assert!(
            caption_text.len() >= 5 && caption_text.len() < 15,
            "Comment length should be between 6 and 14"
        );
    }
}
