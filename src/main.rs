fn test(a: u32) {
    println!("A:{}", a);
}

fn no_ref() {
    let b = 1;
    test(b);
    println!("B:{}", b);
}

fn takes_ownership(some_string: String) {
    println!("Hello {}, I took you", some_string);
}

fn takes_ref() {
    let s = String::from("hello");
    println!("S:{}", s);
    takes_ownership(s);
}

fn tried_taking_ownership(tried :u32) {
    println!("Tried:{} to take you", tried);
}

fn copy_by_default() {
    let a = 3;
    tried_taking_ownership(a);
    println!("A:{}, still have you", a);
}

fn pass_ref(string_ref: &String) {
    println!("Ref:{}", string_ref);
}

fn pass_reference() {
    let mut s = String::from("Hello");
    pass_ref(&s);
    println!("S:{}", s);
    s.push_str(" world!");
    println!("S modified:{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(" World!");
}

fn change_reference() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("S changed:{}", s);
}

fn multiple_references() {
    let mut s = String::from("Mutable String");
    let a = &s;
    let b = &s;
    println!("A:{}", a);
    println!("B:{}", b);
    s.push_str(", mutated!");
    println!("S:{}", s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
	if item == b' ' {
	    return i;
	}
    }
    s.len()
}

fn first_word_test() {
    let mut s = String::from("Hello World!");
    let s_len = first_word(&s);
    println!("Len:{}", s_len);
    s.clear();
    println!("Len After Clearing:{}", s_len);
}

// str => string slice
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
	if item == b' ' {
	    return &s[0..i];
	}
    }
    &s[..]
}

// str => string slice
fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
	if item == b' ' {
	    return &s[i+1..];
	}
    }
    &s[..]
}

fn first_word_slices_test() {
    let mut s = String::from("Hello Cruel World");
    let fword = first_word_slice(&s);
    println!("Fword:{}", fword);
    let bword = second_word_slice(&s);
    println!("Bword:{}", bword);
    s.clear();
    s.push_str("!!!");
    println!("S:{}", s);
}

fn main() {
    no_ref();
    takes_ref();
    copy_by_default();
    pass_reference();
    change_reference();
    multiple_references();
    first_word_test();
    first_word_slices_test();
}
