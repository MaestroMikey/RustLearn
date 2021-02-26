fn tuple1() -> (i32, i32) {
    (3, 8)
}


fn main() {
    let (x, y) = tuple1();

    if y < 5 {
        println!("Less than 5");
    } else if x == 5 {
        println!("On the money!");
    } else {
        println!("Bigger than 5.");
    }
}
