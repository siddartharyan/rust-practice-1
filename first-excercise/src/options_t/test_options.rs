pub fn options_example() {
    let output = find_first_a(String::from("here I'm a"));
    match output {
        Some(ans) => println!("found at index {}", ans),
        None => println!("not found"),
    }
}

fn find_first_a(s: String) -> Option<u32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as u32);
        }
    }
    return None;
}
