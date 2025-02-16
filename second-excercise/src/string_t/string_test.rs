pub fn test_string() {
    test_string_and_slice_1();
    test_string_and_slice_2();
}

fn test_string_and_slice_1() {
    let s = String::from("hello world");
    let first_w = get_first_word(&s);
    println!("first word for {} is {}", s, first_w);
}

fn test_string_and_slice_2() {
    let mut s = String::from("Hello world");
    let first_w = get_first_word_slice(&s);
    println!("first word using slice for {} is {}", s, first_w);
}

fn get_first_word_slice(s: &String) -> &str {
    let mut index: usize = 0;
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        index += 1;
    }
    return &s[0..index];
}

fn get_first_word(s: &String) -> String {
    let mut ans = String::new();
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        ans.push(i);
    }
    return ans;
}
