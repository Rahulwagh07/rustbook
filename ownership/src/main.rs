// ownership--managing heap data

//Ownership Rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    copy_trait();
    mutable_borrow();
    slice_type();

   //slices
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    
}

// fn ownership_move() {
//     let s1 = String::from("hello"); //s1 owns the string
//     let s2 = s1; // ownership moves from s1 to s2
//     println!("{s1}"); //error..s1 is no longer valid
//     println! {"{s2}"};
// }

fn copy_trait() {
    //Copy : stack only data
    // duplicate it
    let x = 5;
    let y = x;

    println! {"{x}"}
    println! {"{y}"};
}

fn double_free_error() {}

fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
                   // x does NOT move into the function,
                   // so it's okay to use x afterward.
} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn references_and_borrowing() {
    //reference : address of val... reference don't take ownership

    let s = String::from("hello");
    let r = &s;

    // A reference is like a pointer in that it’s an address
    //  we can follow to access the data stored at that address; 
    //  that data is owned by some other variable. 
    //  Unlike a pointer, a reference is guaranteed 
    //  to point to a valid value of a particular 
    //  type for the life of that reference
}

fn immutable_borrow() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;

    //work because we are only doing read here
    println!("{} {}", r1, r2);
}

fn mutable_borrow() {
    let mut s = String::from("hello");

    let r = &mut s;
    r.push_str(" world");

    let r2 = &mut s;
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // s is destroyed after scope

//correct version
fn no_dangle -> &String {
    String::from("hello");
}

fn slice_type() {
    //slice don't take ownership

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..];

    println!("{} {}", hello, world);
}

fn memory_layout() {
    //static memory (string literal) ..and it is immutable
    let static_str: &'static str = "hello";

    //heap memory (owned String)
    let heap_string: String = String::from("hello");

    // stack variable holding reference info
    let stack_ref: &str = &heap_string;

    println!("static_str: {}", static_str);
    println!("heap_string: {}", heap_string);
    println!("stack_ref: {}", stack_ref);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if  item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn clone_method() {
    //clone -> give deeply copy the heap data of the String, not just the stack data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

 

//ownership..
// one owner
// move on assignment
// drop on scope end



//borrowing..
// &T -> read
// &mut T -> write 
// no mix allowed

//slices..
// borrowed part of data
// no ownership
//works for both String and &str --// String owns the data, &str borrows the data
