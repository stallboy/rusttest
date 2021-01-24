use std::collections::HashMap;

pub fn test() {
    test_read_update();
    test_update_based_on_old_value();
}

fn test_read_update() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 以下出错，field_name, field_value的对象所有权已经在map里。
    // println!("{}", field_name);

    let key = String::from("Favorite color");
    let score = map.get(&key);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

fn test_update_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}