//Topic: Functions
//
//Program Requirements:
//* Displays your first and last name

//Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// Use the println! macro to display messages to the terminal.

fn main() {
  firstname();
  lastname();
  ifelse1();
  ifelse2();
}

fn firstname() {
    println!("Michael");
}

fn lastname() {
    println!("Mesa");
}

//Topic: Flow control using if...else
//
// Program requirements:
// * Displays a message based on th value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
//
// Notes:
// * use a variable set to either true or false
// * Use an if...else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn ifelse1() {
  let a = true;

  if a == true {
    println!("Hello");

  } else {
    println!("Goodbye");
  }

}

// Topic: Flow control using if..else if..else
//
//Program Requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//  is > 5, < 5, or == 5, respectively
//
//Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to tdetemrine which message to display
// * Use the println! macro to display messages to the terminal

fn ifelse2() {
  let a = 5;

  if a > 5 {
    println!(">5");
  } else if a < 5 {
    println!("<5");
  } else {
    println!("=5");
  }
}