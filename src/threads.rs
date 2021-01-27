use std::thread;

#[test]
fn test_thread_move() {
    let v = vec![1, 2, 3];
    // Rust 会 推断 如何捕获 v，因为 println! 只需要 v 的引用，闭包尝试借用 v。
    // 然而这有一个问题：Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效。
    // 所以这里要加上move
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}


use std::sync::mpsc;
use std::time::Duration;

#[test]
fn test_send_recv() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // val 已经被send转移走所有权了。这里不能再用
        // println!("Send: {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[test]
fn test_multisend_recv(){

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}