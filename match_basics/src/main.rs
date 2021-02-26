use std::borrow::Borrow;

fn main() {
    let some_bool = true;
    match some_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }
    match1();
    match2();
}

//TopicL Decision making with match
//
// Program Requirements:
// * Display "it's true" or "it's false" based on the value
// a boolean variable
//
// Notes:
// * Use a variable set to either or false
// * Use a match expression to determine which message to display

fn match1() {
    let kstate = false;

    match kstate {
        true => println!("It's true."),
        false => println!("It's false."),
    }

}


// Topic: Decision making with match
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
// the value of a variable is 1, 2, 3, or some other value,
// respectively
//
// Notes:
// * Use a variable set to any integer
// * Yse a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn match2() {
    let cannabis = 420;
    match cannabis {
        1 => println!("Not high enough."),
        2 => println!("Not high enough."),
        3 => println!("Not high enough"),
        _ => println!("Maybe high enough"),

    }
}