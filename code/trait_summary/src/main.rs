use crate::aggregator::{NewsArticle, Summary, Tweet, notify};

mod aggregator;

fn main() {
    let news_article = NewsArticle {
        headline: "Look".to_string(),
        location: "Shan Xi".to_string(),
        author: "Han ZhuoXian".to_string(),
        content: "Look at the man".to_string(),
    };

    println!("news article: {}", news_article.summarize());

    let tweet = Tweet {
        username: "zhuoxian".to_string(),
        content: "I like you!".to_string(),
        reply: false,
        retweet: false,
    };

    println!("tweet: {}", tweet.summarize());

    notify(&news_article);
    notify(&tweet);
}
