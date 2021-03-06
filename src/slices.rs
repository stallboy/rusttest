// SliceType :
// [ Type ]
//
// A slice is a dynamically sized type representing a 'view' into a sequence of elements of type T.
// The slice type is written as [T].
// To use a slice type it generally has to be used behind a pointer for example as:
//
// &[T], a 'shared slice', often just called a 'slice', it doesn't own the data it points to, it borrows it.
// &mut [T], a 'mutable slice', mutably borrows the data it points to.
// Box<[T]>, a 'boxed slice'

// Pointer types to DSTs are sized but have twice the size of pointers to sized types
// ---- Pointers to slices also store the number of elements of the slice.
// ---- Pointers to trait objects also store a pointer to a vtable.

// slice的结构是 ptr，len, “string slice” is written as &str
// reference的结构是只有一个ptr
// String的结构是ptr，cap，len
// 从ownership的角度看variable拥有value，但reference和slice都没有value的ownership

pub fn test() {
    test_slice();
    test_string_literals_are_slices();

    test_idx_ok();
    test_slice_compile_err();
}


fn test_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    //TODO str到底是什么类型，为什么不能直接用s[0..5]
    let world = &s[6..11];

    println!("{} {}", hello, world);


    let a = [1, 2, 3, 4, 5];

    // array的slice
    let slice: &[i32] = &a[1..3];
    println!("{}", slice[0]);

    let slice2: &[i32] = &a[0..0];
    println!("slice with {} items", slice2.len());
}

// string literals 作为binary里data里的一段数据，这里s的类型是&str
// string literals are immutable; &str is an immutable reference.
fn test_string_literals_are_slices() {
    let s = "Hello, world!";
    println!("{}", s);
}


fn test_idx_ok() {
    let mut s = String::from("hello world");
    let word = first_word_end_index(&s);
    s.clear();
    println!("the first word is: {}", word);
}

fn first_word_end_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}


fn test_slice_compile_err() {
    let /*mut*/ s = String::from("hello world");
    let word = first_word(&s); //A variable s already borrowed as immutable
    // s.clear(); //then s borrowed as mutable
    println!("the first word is: {}", word); //immutable borrow later used here, 这里报错
    //但是这里用的是word，不是s，compiler是如何知道word需要用到s呢？
    //TODO 这个如何理解？

    use_string_slice_as_parameter_is_better(&s);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn use_string_slice_as_parameter_is_better(s: &str) -> &str {
    s
}
