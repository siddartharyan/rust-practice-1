mod enum_t;
mod even;
mod fib;
mod options_t;
mod ownership;
mod print_t;
mod result_t;
mod str;
mod struc;
use crate::enum_t::test_enum::enum_example;
use crate::even::even_example;
use crate::fib::fib_example;
use crate::options_t::test_options::options_example;
use crate::ownership::example;
use crate::print_t::print_time;
use crate::result_t::test_result::result_example;
use crate::second_excercise::hello_world;
use crate::str::str_len_example;
use crate::struc::struct_example;
use second_excercise;

fn main() {
    even_example();
    fib_example();
    str_len_example();
    struct_example();
    enum_example();
    options_example();
    result_example();
    print_time();
    example();
    hello_world();
}
