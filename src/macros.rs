// 宏分2类：
// 声明（Declarative）宏，使用 macro_rules!，
// 三种 过程（Procedural）宏：

// Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// Attribute-like macros that define custom attributes usable on any item
// Function-like macros that look like function calls but operate on the tokens specified as their argument

// 宏能够接受不同数量的参数
// 宏可以在编译器翻译代码前展开，例如，宏可以在一个给定类型上实现 trait 。而函数则不行
// 消极面是宏定义要比函数定义更复杂

// 通用元编程
// let sql = sql!(SELECT * FROM posts WHERE id=1);
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {

#![allow(unused_macros)]

macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
    };
}

extern crate proc_macro;

use proc_macro::TokenStream;

fn f() {}


use foo::show_streams;

#[show_streams]
fn invoke1() {}

pub fn test() {
    println!("xx1");
    // Example: Basic function

// out: attr: ""
// out: item: "fn invoke1() { }"

    // Example: Attribute with input
    #[show_streams(bar)]
    fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

    // Example: Multiple tokens in the input
    #[show_streams(multiple => tokens)]
    fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

    // Example:
    #[show_streams { delimiters }]
    fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"
}