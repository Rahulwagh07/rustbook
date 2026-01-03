use std::any::type_name;

pub fn string_example() {
    utf8_bytes_vs_chars_example();
    using_to_string_method_to_create_a_string_from_string_literal();
    plus_operator_example();
}

fn utf8_bytes_vs_chars_example() {
    let s = "नमस्ते";
    println!("Bytes: {}", s.len());
    for b in s.bytes() {
        print!("{:02X} ", b);
    }

    println!("\n\nChars:");
    println!("Chars: {}", s.chars().count());
    for c in s.chars() {
        println!("{c}");
    }
}

fn using_to_string_method_to_create_a_string_from_string_literal() {
    let data = "initial contents";
    println!("{}", data);
    print_type_of(&data); //string literal &str

    let s = data.to_string();
    println!("{}", s);
    print_type_of(&s); //alloc::string::String

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);
    print_type_of(&s); //alloc::string::String
}

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn string_new_examples() {
    let mut s = String::new();
    s.push_str("hello");
    println!("{s}");
}

fn push_str_example() {
    let mut s = String::from("hello");
    s.push_str(" rust");
    println!("{s}");
}

//appending with single char
fn push_char_example() {
    let mut s = String::from("hi");
    s.push('!'); //push is for one char only
    println!("{s}");
}

fn plus_operator_example() {
    let s1 = String::from("hello ");
    let s2 = String::from("world");

    let s3 = s1 + &s2;

    println!("{s3}");

    //s1 is moved
    //s2 is borrowed
}

fn string_indexing_example() {
    let s = String::from("hello");

    // let c = s[0]; error
    println!("{s}");
}

fn string_slice_example() {
    let s = String::from("hello world");

    // String slicing uses BYTE indices, not character indices.
    // "hello world" is pure ASCII, so:
    // each character = 1 byte
    // which makes slicing safe and predictable here.
    let hello = &s[0..5]; // bytes 0..5 → "hello"
    let world = &s[6..11]; // bytes 6..11 → "world"

    println!("{hello}");
    println!("{world}");

    //slice indices must align with UTF-8 character boundaries.
    // For non-ASCII text (like Hindi or emojis),
    // a single character may take multiple bytes.
    //
    // ex (will panic at runtime):
    // let s = String::from("नमस्ते");
    // let broken = &s[0..1]; // byte index 1 is not a UTF-8 boundary
    //
    // Rust enforces this to prevent invalid or corrupted text.
}
