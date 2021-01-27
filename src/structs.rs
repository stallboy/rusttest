pub fn test() {
    test_struct();
    test_struct_tuple();
    test_debug();
    test_method();
    test_associated_functions();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn test_struct() {
    // 这个结构是分配在栈上的
    let mut u = User {
        username: "meta".to_string(),
        email: "".to_string(),
        sign_in_count: 0xabcdef12,
        active: false,
    };
    u.email = "abc@a.com".to_string();
    println!("{} {} sign_in_count={}, active={}", u.username, u.email, u.sign_in_count, u.active);
}

fn test_struct_tuple() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} {}", black.1, origin.2);
}

// we used the owned String type rather than the &str string slice type.
// This is a deliberate choice because we want instances of this struct to own all of its data
// and for that data to be valid for as long as the entire struct is valid.

// 下面的定义会报错，提示expected lifetime parameter
// struct User2 {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test_debug() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 pretty print is {:?}", rect1);
    println!("rect1 pretty print2 is {:#?}", rect1);
}


impl Rectangle {
    //这是一个method，注意第一个参数是&self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn test_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

impl Rectangle {
    //这是function，不是method
    //区别在第一个参数不是&self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn test_associated_functions() {
    let r = Rectangle::square(3);
    println!(
        "The area of the rectangle is {} square pixels.",
        r.area()
    );
}
