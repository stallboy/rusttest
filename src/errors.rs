use std::fs::File;
use std::{io, fs};
use std::io::Read;

pub fn test() {
    test_panic();
    test_recoverable_error();
}

fn test_panic() {
    let v = vec![1, 2, 3];

    // v[99];
}

fn test_recoverable_error() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => {}
        Err(error) => {
            println!("Problem opening the file: {:?}", error)
        }
    };
}

// 基本实现是enum Result加匹配 match
fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 只能在返回 Result或Option 或者其它实现了 std::ops::Try 的类型的函数中使用 ? 运算符
// A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 有个标准库实现
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
