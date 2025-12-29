struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Creating Different Types with Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Defining Unit-Like Structs
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        //self: the instance the method is being called on
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //mutability of structs
    //entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    //Creating Instances with Struct Update Syntax

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 //we can use .. as the both active and sign_in_count are types that implement the Copy trait
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}"); //{:?} = Debug formatting required Debug trait
    let value = 12;
    println!("{}", value); //display formatting.. use Display trait

    //methods

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() //area method .. similar Rectangle::area(&rect1)
    );

    // Associated Functions
    // All functions defined within an impl
    // block are called associated functions
    // because they’re associated with the type named after the impl.
    let sq = Rectangle::square(3); // not the dot syntax
}
