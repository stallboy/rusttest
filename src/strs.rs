
pub fn test() {
    test_str();
    test_str_cat();
    test_str_idx_not_permit();
    test_str_slice_may_panic();
    test_str_chars();
}


// 再次复习String 的结构如图： 内存布局为ptr，cap，len
// pub struct String {
//     vec: Vec<u8>,
// }
// string slice &str的内存布局为： ptr，len
fn test_str() {
    let data: &str = "initial contents";
    let s: String = data.to_string();
    // 这里有个自动强制转换，&String -> &str
    let s2: &str = &s;
    // 相当于编译器自动做了以下这个事
    let s3: &str = &s[..];
    // 该方法也可直接用于字符串字面值：
    // let s = "initial contents".to_string();
    println!("{}", s2);
}

fn test_str_cat() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");


    let s = s1 + "-" + &s2 + "-" + &s3;
    // 这个会错
    // println!("{} {} {}", s, s1, s2);
    // 但这个不会，因为rust有str 的+，类似于
    // fn add(self, s: &str) -> String {
    // 第一个self，即这里的s1，是move进函数，然后再返回到这里的s。所以s1不能再访问了
    println!("{} {} {}", s, s2, s3);

    // 更复杂的情况，请用fmt，这样才是完全生成了一个新对象
    let ss = format!("{}-{}-{}", s, s2, s3);
    println!("{} {}", s, ss);

}

// 为什么str不支持 Index，因为
// 1，如果按O（1）复杂度的要求，直接返回utf8的编码byte，这不符合人的预期
// 2，如果按人的预期返回char，这不能O（1），只能从头到尾扫描才能知道具体是哪个char，这样也不符合O（1）的预期
fn test_str_idx_not_permit(){
    let s1 = String::from("hello");
    // the trait `Index<{integer}>` is not implemented for `String`
    // let h = s1[0];
    // println!("{}", h);

    let hello = "中文";
    println!("{}.len返回的是utf8 bytes的长度： {}", hello, hello.len());
}

fn test_str_slice_may_panic() {
    let hello = "中文";
    // panicked at 'byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中文`'
    // let s = &hello[0..2];
    // println!("{}", s);

    let s = &hello[0..3];
    println!("{}", s);
}


fn test_str_chars() {
    let hello = "中文";
    for c in hello.bytes() {
        println!("{}", c);
    }

    for c in hello.chars() {
        println!("{}", c);
    }
}