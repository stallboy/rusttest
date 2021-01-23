// Bringing Paths into Scope with the use Keyword
use crate::packages::back_of_house::Appetizer::Soup;

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

// use来导入
// Providing New Names with the as Keyword
use std::fmt::Result;
use std::io::Result as IoResult;

// Using Nested Paths to Clean Up Large use Lists
use std::{cmp::Ordering, io};

// The Glob Operator
use std::collections::*;

fn test_mod() {
    //相对路径
    front_of_house::hosting::add_to_waitlist();
}


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


fn test_refer_to_outside_lib_rand() {
    let x: u8 = rand::random();
    println!("random u8: {}", x);
}