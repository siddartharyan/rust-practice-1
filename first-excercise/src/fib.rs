//0 1 1 2 3
fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut first = 0;
    let mut second = 1;
    let mut ans = 0;

    for _i in 1..n - 1 {
        ans = first + second;
        first = second;
        second = ans;
    }
    return ans;
}

pub fn fib_example() {
    let fib_value = fib(5);
    println!("fib value is {}", fib_value);
}
