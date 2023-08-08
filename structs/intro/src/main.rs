struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("ola@ex.com"),
        sign_in_count: 1,
    };

    user1.email = String::from(
        "
    alameda",
    );

    let user2 = create_user(
        String::from("user2"),
        String::from(
            "
    alameda",
        ),
    );

    let user3 = User {
        username: String::from("user3"),
        ..user1
    };

    println!("user2: {}", user3.email);

    user1.email = String::from(
        "
    alameda",
    );

    println!("user1: {}", user1.email);

    println!("user2: {}", user3.email);

    println!("Hello, world!");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
