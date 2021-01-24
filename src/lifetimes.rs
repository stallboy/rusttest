use std::fmt::Display;

pub fn test() {
    test_borrow_checker();
    test_lifetime_in_function();
    test_lifetime_in_struct();
}

// 编译出错会提示： `x` does not live long enough
fn test_borrow_checker() {
    // let r;                // ---------+-- 'a
    // {                     //          |
    //     let x = 5;        // -+-- 'b  |
    //     r = &x;           //  |       |
    // }                     // -+       |
    // println!("r: {}", r); // ---------+
}


// 以下注释的函数声明会编译报错，返回时个borrowed value但这从哪儿来，声明周期是什么不清楚
// this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `x` or `y`
// fn longest(x: &str, y: &str) -> &str {

// for some lifetime 'a, the function takes two parameters,
// both of which are string slices that live at least as long as lifetime 'a.
// The function signature also tells Rust that the string slice returned from the function
// will live at least as long as lifetime 'a.
//longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_lifetime_in_function() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。要不然它将会是一个悬垂引用
// 以下是不能编译的
// the best fix would be to return an owned data type rather than a reference
// so the calling function is then responsible for cleaning up the value.

// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }


//ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test_lifetime_in_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    // drop(novel); //加入这个就编译出错了
    println!("{}", i.part);
}

// 生命周期省略规则（lifetime elision rules）
// 第一条规则是每一个是引用的参数都有它自己的生命周期参数。
// 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
// 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)
// 那么所有输出生命周期参数被赋予 self 的生命周期。

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//所有的字符串字面值都拥有 'static 生命周期
fn test_static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}