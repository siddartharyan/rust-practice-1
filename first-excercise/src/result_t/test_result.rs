pub fn result_example() {
    let content = std::fs::read_to_string("hello.txt");
    match content {
        Ok(content) => println!("content is {}", content),
        Err(err) => println!("unable to read file {}", err),
    }
}
