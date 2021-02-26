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

fn main() {
  let a = true;

  if a == true {
    println!("Hello");

  } else {
    println!("Goodbye");
  }

}