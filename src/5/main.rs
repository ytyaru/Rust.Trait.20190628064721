/*
 * Rustのトレイト。（C#やJavaでいうインタフェースに類似）
 * CreatedAt: 2019-06-28
 */
fn main() {
    let v = vec![7, 3, 5, 1, 2];
    println!("{:?}: {}", v, largest(&v));
}
fn largest<T>(list: &[T]) -> T {
    let mut max = list[0];
    for i in list.iter() {
        if max < i { max = list[i]; } // error[E0369]: binary operation `<` cannot be applied to type `T`
    }
    max
}

