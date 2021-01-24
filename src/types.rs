// 标量（scalar）和复合（compound）。
// scalar: integer, float, char, bool
// compound: tuple, array
pub fn test() {
    let a: char = 'c';
    let a2: i32 = 123;
    let a3: &i32 = &123;

    let b: (i32, &str) = (123, "abc");
    let c: [i32; 3] = [12, 3, 4];

    // 以下d指向的（）是不是就是rust的nil，null
    // f会用于何处呢？
    let d: () = (); //unit tuple

    // 这个会报错，type annotations needed for `[_; 0]`，因为无法推断出类型
    // let e = [];

    let f: [i32; 0] = [];
    // println!("{}", f);

    //这里断点调试，可看到栈信息，char是4字节，b，c结构也在栈上
    println!("{} {} {}", a, b.1, c[0]);
}