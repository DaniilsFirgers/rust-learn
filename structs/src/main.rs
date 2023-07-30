struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    // we can make it mutable (the ENTIRE instance is mutable then)
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someexample@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremial@example.com");
    // another user using previous user struct

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // or

    let user3: User = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0, 0, 0);
    let orogin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
