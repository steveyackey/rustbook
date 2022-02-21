use aggregator::{OtherMedia, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let other = OtherMedia {
        title: String::from("thing"),
        other: String::from("other thing"),
    };

    println!("Other Media: {}", other.summarize());
}
