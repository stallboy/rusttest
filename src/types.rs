pub fn test() {
    let a = 'c';
    let b = (123, "abc");
    let c = [12, 3, 4];

    // 以下d指向的（）是不是就是rust的nil，null
    // f会用于何处呢？
    let d = (); //unit tuple

    // 这个会报错，type annotations needed for `[_; 0]`，因为无法推断出类型
    // let e = [];

    let f: [i32; 0] = [];
    // println!("{}", f);

    //这里断点调试，可看到栈信息，char是4直接，b，c结构在栈上
    println!("{} {} {}", a, b.1, c[0]);
}