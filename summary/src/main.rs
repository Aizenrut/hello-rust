fn main() {
    let tweet = get_summaryable();
    let article = NewsArticle::new(String::from("Igor"), String::from("Rust"), 2022, String::from("Learning Rust"));

    println!("Tweet: {}", tweet.summarize());
    println!("NewsArticle: {}", article.summarize());
}

fn get_summaryable() -> impl Summary {
    Tweet::new(String::from("aizenrut"), String::from("Learning Rust"))
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    user: String,
    text: String
}

impl Tweet {
    fn new(user: String, text: String) -> Tweet {
        Tweet {
            user,
            text
        }
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }
}


pub struct NewsArticle {
    author: String,
    title: String,
    year: i16,
    text: String
}

impl NewsArticle {
    fn new(author: String, title: String, year: i16, text: String) -> NewsArticle {
        NewsArticle {
            author,
            title,
            year,
            text
        }
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("\"{}\" from {}, {}", self.title, self.author, self.year)
    }
}
