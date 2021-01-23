
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。
pub fn test() {
    test_borrow();
    test_reference();
}


fn test_borrow() {
    let a = String::from("hello");
    let b = calculate_len_move(a);
    // a 被函数move走了，所以print时不能用a
    // println!("{} {}", a, b);
    println!("{}", b);

    // 用再返回可以解决这个问题，但太复杂了
    let a = String::from("hello");
    let (a2, b) = calculate_len_move_ret(a);
    println!("{} {}", a2, b);


    // 用reference最简单，这里是borrow 语义，等calculate_len返回，就返回value所有权了。
    let a = String::from("hello");
    let b = calculate_len(&a);
    println!("{} {}", a, b);
}

fn calculate_len_move(s: String) -> usize {
    s.len()
}

fn calculate_len_move_ret(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

// We call having references as function parameters borrowing.
// As in real life, if a person owns something, you can borrow it from them.
// When you’re done, you have to give it back.
fn calculate_len(s: &String) -> usize {
    s.len()
}


// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
fn test_reference() {
    // 连着borrow2次 immutable 是可以的
    let mut a = String::from("hello");
    let b = calculate_len2(&a, &a);
    println!("{} {}", a, b);

    let c = &mut a;
    // 但这里报错，因为a是个immutable borrow，c是个mutable，不允许，因为多线程下会数据竞争 data race
    // println!("{} {}", a, c);
    println!("{}", c);

    let d = &a;
    // 这里报错，跟上面一样，但上一句d不会立马报错，因为后面不用c，那c就已经结束了。
    // a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
    // println!("{}", c);
    println!("{}", d);
}

fn calculate_len2(s: &String, s2: &String) -> usize {
    s.len() + s2.len()
}