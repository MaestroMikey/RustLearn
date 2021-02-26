fn users1() {
    struct User {
        username: String,
        email: String,
        active: bool,
        sign_in_count: u64,

    };

    build_user(String::from("mikeytheinfinite@gmail.com"), String::from("TheMaestro"));

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

struct groceryitem {
    stock: i32,
    price: f64,
}

fn grocery() {
    let cereal = groceryitem {
        stock: 10,
        price: 2.99,
    };
    println!("stock: {}", cereal.stock)
}

enum Flavors {
    grape,
    cherry,
    lime,
}

struct Drink {
    flavor: Flavors,
    ounces: i32,
}

fn print_drink1(drink: Drink) {
    match drink.flavor { //matching struct fields of flavor with the flavors enum
        Flavors::grape => println!("Grape"),
        Flavors::cherry => println!("Cherry"),
        Flavors::lime => println!("Lime"),
    }

    println!("oz: {:?}", drink.ounces);
}

fn main() {
    let cherryish = Drink {
        flavor: Flavors::cherry,
        ounces: 12,
    };

    print_drink1(cherryish);
    grocery();

}