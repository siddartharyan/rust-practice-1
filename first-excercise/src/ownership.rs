pub fn example() {
    ownership_example();
    moving();
}

/*
1. Each value in rust has a owner
2. A value can have only one owner at a time
3. If value goes out of scope the memory will be cleared.
4. Rule for references at one time you can have one mutable reference or multiple immutable references
 */
fn ownership_example() {
    let s = String::from("Hello world");
    println!("here is the string {}", s);
}

fn moving() {
    let s1 = String::from("Hello world");
    let mut s2 = s1;
    //println!("let me print this {}", s1);
    println!("let me print this {}", s2);
    print_str(&mut s2);
    println!("after function call let me print this {}", s2);
}

fn print_str(s: &mut String) {
    println!("printing this in function {}", s);
    alter_string(s);
}

fn alter_string(s: &mut String) {
    s.push_str(" test");
}
