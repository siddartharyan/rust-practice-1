pub fn print_time() {
    let now = chrono::Local::now();
    let utc = chrono::Utc::now();
    println!("time in local tz is {} and utc is {}", now, utc);
}
