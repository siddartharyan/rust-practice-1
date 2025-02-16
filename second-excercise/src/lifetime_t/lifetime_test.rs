pub fn test_lifetime() {
    normal_example();
    case_1();
}

fn normal_example() {
    let s1 = String::from("small");
    let s2 = String::from("longest");
    let longest = longest(s1, s2);
    println!("longest string is {}", longest);
}

fn case_1() {
    let mut longest1;
    let s1 = String::from("long");
    let s2 = String::from("value");
    longest1 = longest_using_ls(&s1, &s2);
    println!("value in lifetype is {}", longest1);
}

fn longest(s1: String, s2: String) -> String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
/*
Life time specifier is the intersection of life time of a and b
 */
fn longest_using_ls<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
