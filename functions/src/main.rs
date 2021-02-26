fn main() {
    println!("What up though!?");
    println!("{}", add(5,6));
    another_function();
}

fn another_function() {
    println!("Real Rap Shit");
}

fn add(a: i32, b: i32) -> i32 { //a: i32, b: i32 are function arguments and parameters
    a + b
}