/*
 * Rustのトレイト。（C#やJavaでいうインタフェースに類似）
 * CreatedAt: 2019-06-28
 */
fn main() {
    let v = vec![7, 3, 9, 1, 2];
    println!("{:?}: {}", v, largest(&v));

    let v = vec!['d', 'f', 'a', 'z', 'c'];
    println!("{:?}: {}", v, largest(&v));

    let v = vec!["d", "f", "a", "z", "c"];
    println!("{:?}: {}", v, largest(&v));

}
//fn largest<T>(list: &[T]) -> T {
//fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &i in list.iter() {
        if max < i { max = i; } // error[E0369]: binary operation `<` cannot be applied to type `T`
    }
    max
}

