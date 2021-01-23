// 所有权（系统）是 Rust 最为与众不同的特性，它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全

// Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 值在任一时刻有且只有一个所有者。
// 当所有者（变量）离开作用域，这个值将被丢弃。

// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
pub fn test() {
    test_copy();
    test_move();
    test_clone();
    test_mut();
}

// Stack-Only Data: Copy
fn test_copy() {
    let a = 123;
    let b = a;
    // ok，这是b = a是copy语义，
    println!("{} {}", a, b);
}

fn test_move() {
    let a = String::from("hello"); //第一层结构是ptr，cap，len
    let b = a; //此时发生shallow copy，拷贝了第一层结构
    // 编译出错，因为value move到b了，a不能再用了
    // 这里虽然语义是move，但debug下还是shallow copy了，也许release下优化打开，会省略掉这个？
    // println!("{} {}", a, b);
    println!("{}", b);
}

fn test_clone() {
    let a = String::from("hello");
    let b = a.clone();
    // clone 是个深拷贝 deep copy，所以a，b都可用
    // 上面的copy，move都是浅拷贝 shallow copy，只不过i32类型标记了Copy trait
    println!("{} {}", a, b);
}

fn test_mut() {
    let a = String::from("hello");
    // 提示a cant borrow as mutable，因为push_str的函数声明是
    // pub fn push_str(&mut self, string: &str) {
    // 需要a是&mut的
    // a.push_str(" world");
    let mut b = a.clone();
    b.push_str(" world");
    println!("{}", b);
}
