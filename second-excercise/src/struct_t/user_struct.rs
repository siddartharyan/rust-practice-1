use crate::trait_t::trait_test::Summary;
pub struct User<'a> {
    pub name: &'a str,
    pub age: u32,
}
impl<'a> User<'a> {
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}
impl<'a> Summary for User<'a> {
    fn Summarize(&self) {
        println!("User's name is {} and age is {}", self.name, self.age);
    }
}
