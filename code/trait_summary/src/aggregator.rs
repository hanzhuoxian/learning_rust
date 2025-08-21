use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {},{}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("News : {}", item.summarize());
}

pub fn notify_bound<T: Summary>(item: &T) {
    println!("News : {}", item.summarize());
}

pub fn notify_bound_display<T: Summary + Display>(item: &T) {
    println!("News : {}", item.summarize());
}

pub fn notify_display(item: &(impl Summary + Display)) {
    println!("News : {}", item.summarize());
}

pub fn some<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("some {} {:?}", t, u);
}

pub fn return_summarize(flag: bool) -> impl Summary {
    Tweet {
        username: "zhuoxian".to_string(),
        content: "you".to_string(),
        reply: true,
        retweet: true,
    }
}
