// Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
// Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
// 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

mod usebox{
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    #[test]
    fn test_box_cannot_share() {
        let a = Cons(5,
                     Box::new(Cons(10,
                                   Box::new(Nil))));
        let _b = Cons(3, Box::new(a));
        // use of moved value: `a`
        // let c = Cons(4, Box::new(a));
    }
}


mod rc {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    // 通过不可变引用， Rc<T> 允许在程序的多个部分之间只读地共享数据。
    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.
    // 如果 Rc<T> 也允许多个可变引用，则会违反第四章讨论的借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致，怎么办？？？
    #[test]
    fn test_rc_can_share_ref() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Cons(3, a.clone());
        println!("count after creating b = {}", Rc::strong_count(&a));
        // 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
        // Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。
        // Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。
        // 通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
        // 当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
        {
            let _c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

//TODO： RefCell这个有点难理解
mod refcell {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_refcell() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}