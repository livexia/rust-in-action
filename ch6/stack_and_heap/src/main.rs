fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}

fn is_strong_2<T: Into<String>>(password: T) -> bool {
    password.into().len() > 5
}

fn main() {
    // String is on the heap
    // 'static &str is on the stack ?
    //   reference is on the stack, but the string view
    //   of str might be to anywhere
    assert_eq!(is_strong("testabc"), true);
    assert_eq!(is_strong("TESTABCDE000".to_string()), true);
    assert_eq!(is_strong(&("abcdeee".to_string())), true);

    assert_eq!(is_strong_2("testabc"), true);
    assert_eq!(is_strong_2("TESTABCDE000".to_string()), true);
    assert_eq!(is_strong_2(&("abcdeee".to_string())), true);

    using_box();
}

fn using_box() {
    let a = Box::new(1);
    let b = Box::new(2);
    let c = Box::new(3);

    // *a get the T, & get the &T
    // pointer should point on the heap
    let a_ptr = &(*a) as *const i32;

    println!("a + b + c = {}", *a + *b + *c);

    drop(a);

    let d = Box::new(1);

    let d_ptr = &(*d) as *const i32;

    println!("d + b + c = {}", *d + *b + *c);

    println!("a_ptr: {a_ptr:p}");
    println!("d_ptr: {d_ptr:p}");
}
