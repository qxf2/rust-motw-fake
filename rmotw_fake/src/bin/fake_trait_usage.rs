//Example program showing the basic usage of generating fake data using Fake Trait

use fake::Fake;

fn generate_fake_social_media_data() {
    let fake_likes = (1..1000).fake::<u32>();
    println!("Number of Likes: {}", fake_likes);

    let fake_shares = (1..100).fake::<u16>();
    println!("Number of Shares: {}", fake_shares);

    let fake_post_content = (5..20).fake::<String>();
    println!("Generated Post Content: {:?}", fake_post_content);

    let fake_comment = (2..5).fake::<String>();
    println!("Generated Comment: {:?}", fake_comment);
}

fn main() {
    generate_fake_social_media_data();
}
