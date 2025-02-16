use std::collections::HashMap;

pub fn test_hashmap() {
    general_hashmap();
    main_tuple_to_hash_map();
}

fn main_tuple_to_hash_map() {
    let mut hash_map = tuple_to_hash_map();
    for (key, value) in hash_map {
        println!("tuple to key value key is {}, value is {}", key, value);
    }
}

fn tuple_to_hash_map() -> HashMap<String, u32> {
    let vector_of_tuples = vec![(String::from("test"), 21), (String::from("test1"), 22)];
    let mut hash_map: HashMap<String, u32> = HashMap::new();
    for (key, val) in vector_of_tuples {
        hash_map.insert(key, val);
    }
    return hash_map;
}

fn general_hashmap() {
    let mut hash_map: HashMap<String, u32> = HashMap::new();
    hash_map.insert(String::from("siddarth"), 23);
    hash_map.insert(String::from("sai"), 23);
    for (key, value) in hash_map {
        println!("key is {} value is {}", key, value);
    }
}
