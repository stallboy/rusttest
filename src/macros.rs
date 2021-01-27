
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
