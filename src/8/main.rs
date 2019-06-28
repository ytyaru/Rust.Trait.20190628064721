/*
 * Rustのトレイト。（C#やJavaでいうインタフェースに類似）
 * CreatedAt: 2019-06-28
 */
fn main() {
    let p1 = Pair { x:1, y:2 };
    let p2 = Pair { x:2, y:1 };
    let p3 = Pair::new(0,0);
    p1.cmp_display();
    p2.cmp_display();
    p3.cmp_display();
//    let p4 = Pair::new(Pair::new(0,0), Pair::new(0,0));
//    p4.cmp_display(); // error[E0599]: no method named `cmp_display` found for type `Pair<Pair<{integer}>>` in the current scope
}
pub struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self { Self { x, y } }
}
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y { println!("The largest number is x = {}", self.x); }
        else { println!("The largest number is y = {}", self.y); }
    }
}

