/*
 * Rustのトレイト。（C#やJavaでいうインタフェースに類似）
 * CreatedAt: 2019-06-28
 */
fn main() {
    let n = NewsArticle {
        headline: "題名".to_string(),
        location: "場所".to_string(),
        author: "著者".to_string(),
        content: "内容".to_string(),
    };
    println!("{}", n.summarize());

    let t = Tweet {
        username: "ユーザ名".to_string(),
        content: "ツイート内容".to_string(),
        reply: false,
        retweet: false,
    };
    println!("{}", t.summarize());
}
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
/*
impl Summary for NewsArticle {
    fn summarize(&self) -> String { format!("{}, by {} ({})", self.headline, self.author, self.location) }
}
*/
//impl Summary for NewsArticle {}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String { format!("{}", self.author) }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
/*
impl Summary for Tweet {
    fn summarize(&self) -> String { format!("{}: {}", self.username, self.content) }
}
*/
//impl Summary for Tweet {}
impl Summary for Tweet {
    fn summarize_author(&self) -> String { format!("{}", self.username) }
}
