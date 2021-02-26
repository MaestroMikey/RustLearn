fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    loop1();
    loop2();
    loop3();
    loop4();
    loop5();
}

fn loop1() {
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        println!("{:?}", a);
        a = a + 1;
    }
}

fn loop2() {
    let mut a = 0;
    while a != 5 {
        println!("{:?}", a);
        a = a + 1;
    }
}

//repeating a task multiple times

fn loop3() {
    let mut a = 3;
    loop {
        if a == 0 {
            println!("Blast Off!");
            break;
        }
        println!("{}", a);
        a = a - 1;
    }
}

fn loop4() {
    let mut a = 1;
    loop {
        println!("{}", a);
        if a == 4 {
            break;
        }
        a = a + 1;
    }
}

fn loop5() {
    let mut a = 5;
    while a != 0 {
        println!("{}", a);
        a = a - 1;
    }
    println!("Done!");
}
