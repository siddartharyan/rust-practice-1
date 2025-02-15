pub fn str_len_example() {
    let s = String::from("hello world!");
    println!("length is {}", calculate_length(&s));
}

fn calculate_length(s: &str) -> usize {
    s.chars().count()
}
