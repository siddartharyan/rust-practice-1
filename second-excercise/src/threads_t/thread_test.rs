use std::{sync::mpsc, thread, time::Duration};

pub fn test_thread() {
    thread_test();
    thread_test_1();
    thread_test_2();
    test_channel();
    test_sum_threads();
}

fn thread_test() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("child process here {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("main thread logs {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_test_1() {
    let t = thread::spawn(|| {
        for i in 1..10 {
            println!("child process here {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    t.join().unwrap();

    for i in 1..10 {
        println!("main thread logs {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
/*
move the ownership of vector to thread
 */
fn thread_test_2() {
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        println!("here in thread {:?}", v);
        thread::sleep(Duration::from_millis(1));
    });
    thread::sleep(Duration::from_millis(1));
}

fn test_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("here test");
        tx.send(s).unwrap();
    });

    let data = rx.recv().unwrap();
    println!("received {}", data);
}

fn test_sum_threads() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let mut val = 0;
        for i in 1..1001 {
            val += i;
            //thread::sleep(Duration::from_millis(2));
        }
        tx1.send(val).unwrap();
    });

    thread::spawn(move || {
        let mut val = 0;
        for i in 1001..2001 {
            val += i;
            //thread::sleep(Duration::from_millis(2));
        }
        tx.send(val).unwrap();
    });

    let mut final_val = 0;
    for r in rx {
        final_val += r;
    }
    println!("sum is {}", final_val);
}
