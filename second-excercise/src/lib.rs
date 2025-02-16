mod generic_t;
mod hashmap_t;
mod itr_t;
mod lifetime_t;
mod string_t;
mod struct_t;
mod threads_t;
mod trait_t;
mod vector_t;
use crate::generic_t::generic_test::test_generic;
use crate::hashmap_t::hashmap_test::test_hashmap;
use crate::itr_t::iterator_test::test_iterator;
use crate::lifetime_t::lifetime_test::test_lifetime;
use crate::string_t::string_test::test_string;
use crate::struct_t::struct_test::test_struct;
use crate::threads_t::thread_test::test_thread;
use crate::trait_t::trait_test::test_trait;
use crate::vector_t::vector_test::test_vector;

pub fn hello_world() {
    println!("Hello world from second package");
    test_vector();
    test_hashmap();
    test_iterator();
    test_string();
    test_generic();
    test_trait();
    test_struct();
    test_lifetime();
    test_thread();
}
