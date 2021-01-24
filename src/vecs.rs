pub fn test() {
    test_vec();
    test_vec_get();
    test_vec_cannot_push_when_get();
    test_foreach();
}

fn test_vec() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{}", v[0]);
}

//get 返回Option<&T>，而v[index]会导致panic
fn test_vec_get() {
    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = v.get(100);
    println!("{}", does_not_exist.unwrap_or(&10000));
    // let does_not_exist = &v[100];
}

//为什么ref 第一个元素后，vec不能再增加元素，因为再增加后，可能导致vec所有元素复制到新的内存地址
//如果first这个ref仍然可用的话，就出错了。
fn test_vec_cannot_push_when_get() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = /*&*/v[0]; //copy 之后是ok的，但解掉注释用ref就compile error了
    v.push(6);
    println!("The first element is: {}", first);
}

fn test_foreach() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

