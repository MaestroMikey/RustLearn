enum Colors {
    Red,
    Purple,
    Green,
    Yellow
}

fn print_color(favorite_color: Colors) {
    let favorite_color = Colors::Purple;
    match favorite_color {
        Colors::Purple => println!("This is my favorite color!"),
        Colors::Red => println!("This is not my favorite color"),
        Colors::Green => println!("Green is not a creative color"),
        Colors::Yellow => println!("Lemons?"),
    }
}

fn main() {
    print_color(Colors::Purple);
}