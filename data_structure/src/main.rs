fn main() {
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

let user1;