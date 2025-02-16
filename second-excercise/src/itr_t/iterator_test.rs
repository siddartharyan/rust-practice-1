pub fn test_iterator() {
    immutable_iterator();
    mutable_iterator();
    into_iter_test();
    consumer_adaptor_test();
    test_iterator_1();
    test_iterator_2();
    test_iterator_3();
}

fn mutable_iterator() {
    let mut v = vec![1, 2, 3];
    let mut itr = v.iter_mut();
    for val in itr {
        *val = *val * 2;
    }
    println!("values are {:?}", v);
}

fn immutable_iterator() {
    let v = vec![1, 2, 3, 4];
    let itr = v.iter();
    for val in itr {
        println!("in immutable itr {}", val);
    }
    println!("{:?}", v);
    println!();
    println!("using while loop");
    let mut itr1 = v.iter();
    while let Some(val) = itr1.next() {
        println!("value using whileloop {}", val);
    }
}
/*
This has a special property where ownership of v is transferred to itr
 */
fn into_iter_test() {
    println!();
    let v = vec![1, 2, 3, 4];
    let itr = v.into_iter();
    for val in itr {
        println!("into itr value is {}", val);
    }
}
/*
consume the original iterator
 */
fn consumer_adaptor_test() {
    let v = vec![1, 2, 3, 4];
    let itr = v.iter();
    let sum: u32 = itr.sum();
    println!("sum value is {}", sum);
}

/*
iterator adaptor gives a new iterator
 */
fn test_iterator_1() {
    let v = vec![1, 2, 3, 4];
    let itr = v.iter();
    let itr1 = itr.map(|x| x + 1);
    for val in itr1 {
        println!("after operation is {}", val);
    }
}

fn test_iterator_2() {
    let v = vec![1, 2, 3, 4];
    let itr = v.iter();
    let itr1 = itr.filter(|x| *x % 2 == 1);
    println!();
    for x in itr1 {
        println!("filtering values after applying filter is {}", x);
    }
    println!("original array is {:?}", v);
}

fn test_iterator_3() {
    println!();
    let n = 100;
    let mut v: Vec<u32> = Vec::new();
    for i in 1..n {
        v.push(i as u32);
    }

    let itr = v.iter().filter(|x| *x % 2 == 1).map(|x| 2 * x);
    let mut values = Vec::new();
    for val in itr {
        values.push(val);
    }
    println!("values after operation is {:?}", values);
}
