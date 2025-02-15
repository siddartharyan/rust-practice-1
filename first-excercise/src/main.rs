mod enum_t;
mod even;
mod fib;
mod options_t;
mod ownership;
mod print_t;
mod result_t;
mod str;
mod struc;

fn main() {
    even::even_example();
    fib::fib_example();
    str::str_len_example();
    struc::struct_example();
    enum_t::test_enum::enum_example();
    options_t::test_options::options_example();
    result_t::test_result::result_example();
    print_t::print_time();
    ownership::example();
}
