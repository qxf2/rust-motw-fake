//Example program showing the basic usage of generating fake data using Fake Trait

use fake::Fake;

fn generate_fake_social_media_data() -> (u32, u16, String, String) {
    let fake_likes = (1..1000).fake::<u32>();
    let fake_shares = (1..100).fake::<u16>();
    let fake_post_content = (5..20).fake::<String>();
    let fake_comment = (2..5).fake::<String>();

    (fake_likes, fake_shares, fake_post_content, fake_comment)
}

fn main() {
    let (fake_likes, fake_shares, fake_post_content, fake_comment) = generate_fake_social_media_data();

    let output = format!("Number of Likes: {}\nNumber of Shares: {}\nGenerated Post Content: {:?}\nGenerated Comment: {:?}", 
    fake_likes, fake_shares, fake_post_content, fake_comment);

    println!("{}", output);
}
