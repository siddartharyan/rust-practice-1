pub fn test_generic() {
    greatest();
}

fn greatest() {
    let a = find_greatest(1, 2);
    let b = find_greatest('a', 'b');
    println!("greater values are {} and {}", a, b);
}

fn find_greatest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
