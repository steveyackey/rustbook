struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        // field init shorthand: when param name and struct names are the same
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
    // If a struct is mutable, all fields are mutable within it
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("test@example.com");
    println!("Hello, {}!", user1.username);

    // Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.email);
    user1.email = String::from("last@example.com");

    let user3 = User { ..user2 };

    println!("{}", user2.email);

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // Unit like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

