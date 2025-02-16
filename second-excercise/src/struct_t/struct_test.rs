use crate::struct_t::user_struct::User;
use crate::trait_t::trait_test::Summary;

pub fn test_struct() {
    test_struct_1();
}

fn test_struct_1() {
    let mut u = User {
        name: &String::from("siddartha"),
        age: 23,
    };
    let s = &String::from("sid");
    u.set_name(s);
    u.set_age(20);
    u.Summarize();
}
