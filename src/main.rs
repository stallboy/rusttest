mod types;
mod ownerships;
mod references;
mod slices;
mod structs;
mod enums;
mod modules;
mod mods;
mod vecs;
mod strs;
mod hashmaps;
mod errors;
mod lifetimes;
mod macros;

mod xx {
    pub const A: i32 = 123;
    pub static B: i32 = 123;
}

mod yy {
    use crate::xx;

    fn f() {
        println!("{} {}", crate::xx::A, super::xx::A);
        println!("{}", xx::A);
    }
}

fn main() {
    // //lib.rs文件名的作用
    // let guess = rusttest::Guess::new(99);
    types::test();
    ownerships::test();
    references::test();
    slices::test();
    structs::test();
    enums::test();
    modules::test();
    //mods目录下mod.rs文件名的作用。
    mods::test();
    vecs::test();
    strs::test();
    hashmaps::test();
    errors::test();
    lifetimes::test();
    macros::test();
}


fn func_in_main() -> i32 {
    42
}

#[cfg(test)]
#[test]
fn test_func_in_main() {
    assert_eq!(func_in_main(), 42);
}