use std::ops::Add;
use core::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn test_operator_overloading() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}


trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[test]
fn test_super_traits() {
    let p = Point { x: 1, y: 123 };
    println!("{}", &p);
    p.outline_print();
}



fn generic<T>(t: T) {
    // --snip--
}
// 实际上是被处理成
fn generic2<T: Sized>(t: T) {
    // --snip--
}

// 除非你特意加上?Sized,来放宽这个限制,,?Sized trait bound它可以读作"T 可能是也可能不是 Sized 的"
// 注意我们将 t 参数的类型从 T 变为了 &T：因为其类型可能不是 Sized 的，
// 所以需要将其置于某种指针之后。在这个例子中选择了引用。
fn generic3<T: ?Sized>(t: &T) {
    // --snip--
}

