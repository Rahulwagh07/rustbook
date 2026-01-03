// Vectors allow you to store more than one value
// in a single data structure that puts all the values
//  next to each other in memory.
// Vectors can only store values of the same type
mod hashmap;
mod string;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // vec_examples();
    //string::string_example();
    hashmap::hashmap_example();
}

fn vec_examples() {
    let mut v: Vec<i32> = Vec::new();

    //Rust conveniently provides the vec! macro,
    //which will create a new vector that holds the values you give it
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //read element

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(8);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //get method is passed an index that is outside the vector,
    //it returns None without panicking.
    let does_not_exists = &v.get(100);
    println!("Value of does_not_exists is: {:?}", does_not_exists);

    for i in &v2 {
        println!("{i}");
    }

    enum_to_multiple_types_example();
}

fn enum_to_multiple_types_example() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(val) => println!("Int: {val}"),
            SpreadsheetCell::Float(val) => println!("Float: {val}"),
            SpreadsheetCell::Text(val) => println!("Text: {val}"),
        }
    }
}
