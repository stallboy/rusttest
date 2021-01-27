// 3种情况下使用Box
// 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候


use crate::boxes::List::{Cons, Nil};

// 报错：help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[test]
fn test() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;


impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn test_deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct Compound {
    a: i32,
    b: i32,
}

// Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：
// 当 T: Deref<Target=U> 时从 &T 到 &U。
// 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
// 当 T: Deref<Target=U> 时从 &mut T 到 &U。
#[test]
fn test_deref_struct() {
    let x = Compound {
        a: 5,
        b: 2,
    };
    let y = MyBox::new(x);

    //x的所有权在y里了，x不能再用了
    // assert_eq!(5, x.a);
    assert_eq!(5, y.a);
    assert_eq!(2, y.b);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // 看看std::mem::drop的实现吧，是个空函数
    // pub fn drop<T>(_x: T) {}，所有权移过去，然后等函数返回的时候_x就被回收了。
    drop(m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}