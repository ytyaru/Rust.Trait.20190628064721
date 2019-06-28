/*
 * Rustのトレイト。（C#やJavaでいうインタフェースに類似）
 * CreatedAt: 2019-06-28
 */
fn main() {
    let v = vec![7, 3, 9, 1, 2];
    println!("{:?}: {}", v, largest_copy(&v));
    let v = vec!['d', 'f', 'a', 'z', 'c'];
    println!("{:?}: {}", v, largest_copy(&v));
    let v = vec!["d", "f", "a", "z", "c"];
    println!("{:?}: {}", v, largest_copy(&v));

    let v = vec![7, 3, 9, 1, 2];
    println!("{:?}: {}", v, largest_clone(&v));
    let v = vec!["d", "f", "a", "z", "c"];
    println!("{:?}: {}", v, largest_clone(&v));

    let v = vec![7, 3, 9, 1, 2];
    println!("{:?}: {}", v, largest(&v));
    let v = vec!["d", "f", "a", "z", "c"];
    println!("{:?}: {}", v, largest(&v));

}
fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &i in list.iter() {
        if max < i { max = i; }
    }
    max
}
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut max = &list[0];
    for i in list.iter() {
        if max < i { max = i; }
    }
    max.clone()
}
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for i in list.iter() {
        if max < i { max = i; }
    }
    max
}
/*
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut max = &list[0];
    for i in list.iter() {
        if max < i { max = i; }
    }
    max
}
fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let list_ = list.clone();
    let mut max = &list_[0];
    for i in list_.iter() {
        if max < i { max = i; }
    }
    max.clone()
}
*/

