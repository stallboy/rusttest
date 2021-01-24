// crate 是一个二进制项或者库。crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块
// 包（package） 是提供一系列功能的一个或者多个 crate。一个包会包含有一个 Cargo.toml 文件，阐述如何去构建这些 crate。
// 模块 让我们可以将一个 crate 中的代码进行分组，以提高可读性与重用性。

pub fn test() {
    test_mod();
    test_struct_field_default_private();
    test_enum_variants_default_public();
    test_refer_to_outside_lib_rand();
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to wait list")
        }
    }
}

// pub use重导出名称
// Re-exporting Names with pub use
pub use front_of_house::hosting;

fn test_mod() {
    //相对路径
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


// Bringing Paths into Scope with the use Keyword
use crate::modules::back_of_house::Appetizer::Soup;

// use来导入
// Providing New Names with the as Keyword
use std::fmt::Result;
use std::io::Result as IoResult;

// Using Nested Paths to Clean Up Large use Lists
use std::{cmp::Ordering, io};

// The Glob Operator
use std::collections::*;


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Eq, PartialEq)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 结构里的字段默认是private
fn test_struct_field_default_private() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// 枚举成员默认public
fn test_enum_variants_default_public() {
    let order1 = back_of_house::Appetizer::Soup;
    println!("{}", order1 == Soup);
}

// module tree和文件的tree是不同的，这里rust没有跟java一样设计，让file tree就是module tree，
// java的是在各文件里在类前面声明package xxx; （package等价于这里的mod），
// 而rust选择，从lib.rs或main.rs开始构建module tree，mod的构建和比如function的定义不绑定在一起一对一，
// 而是我们可以先构建实现。然后通过mod xx{ pub use yy}的方式构建对外接口 ，
// 对A类人提供A‘接口，对B类人提供B’接口。
// 去看看rand crate的src/lib.rs，src/prelude.rs
fn test_refer_to_outside_lib_rand() {
    let x: u8 = rand::random();
    println!("random u8: {}", x);
}