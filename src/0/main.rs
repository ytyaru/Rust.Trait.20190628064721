/*
 * Rustのトレイト。（C#やJavaでいうインタフェースに類似）
 * CreatedAt: 2019-06-28
 */
fn main() {
    println!("Hello Rust trait!!");
}
pub trait Summary {
    fn summarize(&self) -> String;
}
