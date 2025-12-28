fn main() {
    // constant_in_rust();
    // shadowing_in_rust();
    // integer_overflow_example();
    // isize_and_usize_integers();
    // floating_point_types();
    // boolean_types();
    // character_type();
    // compound_types();
    // unit_tuple();
    // array_type();
    // functions_in_rust();
    control_flow_in_rust()
}

fn constant_in_rust() {
    // Constants aren’t just immutable by default—they’re always immutable.
    // You declare constants using the const keyword instead of the let keyword

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}

fn shadowing_in_rust() {
    //shadowing -> declaring a new variable with the same name using let,
    //so the new variable temporarily replaces (hides) the old one within the current scope.

    let x = 7;
    let x = x + 2;

    println!("Value of x is: {x}");

    {
        let x = x * 2;
        println!("Value of x in inner scope: {x}");
    }

    println!("value of x is: {x}");
}

fn scalar_data_types_in_rust() {
    //rust is statically typed

    //A scalar type represents a single value.
    //Rust has four primary scalar types: integers,
    // floating-point numbers, Booleans, and characters

    // 1)integers
    //-> number without a fractional component
    //-> signed integer types start with i instead of u
    //pointer-sized integers -> isize and usize
}

fn integer_overflow_example() {
    let x: u8 = 255;

    // wrapping → loop back to start
    let a = x.wrapping_add(1);
    println!("wrapping_add: {}", a); // 0

    // checked → tell me if it failed
    let b = x.checked_add(1);
    println!("checked_add: {:?}", b); // None

    // overflowing → give result + warning
    let (c, overflowed) = x.overflowing_add(1);
    println!("overflowing_add: {}, overflowed: {}", c, overflowed); // 0, true

    // saturating → do not go past the limit
    let d = x.saturating_add(1);
    println!("saturating_add: {}", d); // 255
}

fn isize_and_usize_integers() {
    let data = [5, 6, 7, 8];
    let i: isize = 1;
    let offset: isize = -1;

    let new_i = (i + offset) as usize; // convert to index
    println!("{}", data[new_i]); //  0 idx return 5
}

fn floating_point_types() {
    //f32 and f64
    //default f64

    let x = 12.32; //f64
    let y: f32 = 12.33;

    print_type(&x);
    print_type(&y);
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn boolean_types() {
    let is_user = true;
    print_type(&is_user); //bool
}

fn character_type() {
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻'; //single quote

    print_type(&heart_eyed_cat); //char
}

fn compound_types() {
    //Compound types can group multiple values into one type

    // rust has two-> tuples and arrays

    //tuples
    // A tuple is a general way of grouping together a number
    // of values with a variety of types into one compound type.
    // Tuples have a fixed length: Once declared,
    // they cannot grow or shrink in size

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //destructure tuples
    let (x, y, z) = tup;

    println!("The values of y is : {y}");

    //access by directly using period
    let second_val = tup.1;
    println!("Second value is: {second_val}");
}

fn unit_tuple() {
    //The tuple without any values has a special name, unit.
    //This value and its corresponding type are
    // both written () and represent an empty value
    //or an empty return type. Expressions implicitly
    //return the unit value if they don’t return any other value.

    let a = (); // unit value
    say_hello(); // returns ()

    println!("{:?}", a); // ()
}

fn say_hello() {
    // no return value → returns ()
    println!("Hello!");
}

fn array_type() {
    //every ele in array have same type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //initial val 3 with 5 elements
    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    println!("first: {first}")
}

fn functions_in_rust() {
    //rust uses snake case-> all letter are
    //lowercase and underscores separated words

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value

    //If you add a semicolon to the end of an expression,
    // you turn it into a statement,
    //and it will then not return a value

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let res = five();

    println!("result is: {res}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1; //error... changed it from expression to a statement by adding ;
}

fn control_flow_in_rust() {
    let condition = true;
    // let number: number | String = if condition { 5 } else { "six" }; //rust does not have union type..

    // println!("The value of num is: {number}");

    let number = if condition {
        "5".to_string()
    } else {
        "six".to_string()
    };

    println!("The value of number is: {number}");
}
