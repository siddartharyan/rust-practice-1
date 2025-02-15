fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}

pub fn even_example() {
    let even_value = is_even(20);
    println!("is even value is {}", even_value);
}
