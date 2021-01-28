// Traits: Defining Shared Behavior
// 只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。
// 不能为外部类型实现外部 trait
// This restriction is part of a property of programs called coherence,
// and more specifically the orphan rule
// 这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。
// 没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。

// 以上就是trait和java interface机制的主要区别。实现没必要在类声明的地方。而是可以逐步添加。


#[test]
fn test_imp_trait_static_dispatch() {
    notify(123);
    notify('c');
}


// imp Trait 等价于notify2里的  trait bound
fn notify(item: impl ToString) {
    println!("Breaking news! {}", item.to_string());
}

fn notify2<T: ToString>(item: T) {
    println!("Breaking news! {}", item.to_string());
}


#[test]
fn test_find_max_copy() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 注意看跟前一个的区别，这里用reference
#[test]
fn test_find_max_reference() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_ref(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}



trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() + "xxx" }
}

//调试可看到a结构为 pointer + vtable（vtable里有3个成员，另外两个是啥？）
//不能如下声明因为，不知道Printable的Size
// fn print(a: Printable) {
fn print(a: &dyn Printable) {
    let x = 0x1234abcd; //as marker
    println!("{} {}", a.stringify(), x);
}

#[test]
fn test_trait_as_param() {
    print(&10);
}


