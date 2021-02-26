#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // implementing rectangle
    fn area(&self) -> u32 { // move function inside implemented body, signature is &self
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle Area = {}", rect1.area()); // call area method on Rectangle instance with
    // method syntax, going after rect1 instance
}