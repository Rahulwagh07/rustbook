#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn option_enum() {
    // option enum : value could be something or could be nothing
    let some_number = Some(5);
    let some_char = Some('c');

    let absent_number: Option<i32> = None;

    println!("{some_number:?}, {some_char:?}, {absent_number:?}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn value_in_cents_new(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn if_let_example() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    Some(format!("{state:?} is a us state quarter"))
}

fn main() {
    // IpAddrKind + struct
    let home_struct = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{home_struct:?}");

    //enum with data
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{home:?}");
    println!("{loopback:?}");

    // Coin example
    let coin = Coin::Quarter(State::Alaska);

    println!("Value: {}", value_in_cents(coin));

    let coin2 = Coin::Penny;
    println!("Value new: {}", value_in_cents_new(coin2));

    let coin3 = Coin::Quarter(State::Alabama);
    println!("{:?}", describe_state_quarter(coin3));

    // option
    option_enum();
    if_let_example();
}
