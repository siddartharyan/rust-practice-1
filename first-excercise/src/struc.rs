struct Rect {
    height: u32,
    width: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

pub fn struct_example() {
    let rect = Rect {
        width: 12,
        height: 10,
    };
    println!("value of area is {}", rect.area());
    println!("value of perimeter is {}", rect.perimeter());
}
