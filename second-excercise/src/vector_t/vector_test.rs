pub fn test_vector() {
    let mut values: Vec<u32> = Vec::new();
    populate_vector(&mut values);
    filter_even_values(&mut values);
    println!("values {:?}", values);
}

fn populate_vector(values: &mut Vec<u32>) {
    for i in 1..100 {
        values.push(i as u32)
    }
}

fn filter_even_values(values: &mut Vec<u32>) {
    for i in (0..values.len()).rev() {
        if values[i] % 2 != 0 {
            values.remove(i);
        }
    }
}
