pub fn test() {
    test_enum();
    test_match();
    test_match_placeholder();
    test_if_let();
}

// 枚举的内存布局，见下网页
// https://doc.rust-lang.org/reference/type-layout.html
// 其实就是tag + payload （payload是union）
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(u32),
}


fn test_enum() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let test = IpAddr::V6(0xaabbccdd);
    //可在此断点，查看enum内存布局，
    // 可知tag占u8，但后面的u32会对齐align到4 byte位置。

    let localhost_v4 = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    println!("{}", localhost_v4);


    let a: Option<i32> = Some(123);
    println!("{}", a.unwrap())
}

fn test_match() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{} {}", six.unwrap(), none.unwrap_or(123));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_match_placeholder() {
    let some_u8_value = 3;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //这是找到()的用处了吗?
    }
}

fn test_if_let(){
    let a = 3u8;
    let some_u8_value = Some(a);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three in if let");
    }
}

struct A{
    a:i32,
}

enum B {
    B1,
    B2 = 123
}