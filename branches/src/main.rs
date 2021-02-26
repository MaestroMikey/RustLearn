fn main() {
    let condition = false;
    let number = if condition {
        9
    } else {
        4
    };

    println!("The number is: {}", number);
}