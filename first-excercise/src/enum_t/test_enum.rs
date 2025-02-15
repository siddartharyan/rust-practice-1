enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

pub fn enum_example() {
    let rectangle = Shape::Rectangle(3.15, 2.13);
    println!("area of rectangle is {}", calculate_area(rectangle));
    let circle = Shape::Circle(2.13);
    println!("area of circle is {}", calculate_area(circle));
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(width, height) => width * height,
    };
    return area;
}
