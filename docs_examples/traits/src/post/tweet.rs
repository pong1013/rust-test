use traits::summary::tweet_summary::Tweet;
use traits::Summary;
pub fn tweet_post() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 則新推文：{}", tweet.summarize());
}
