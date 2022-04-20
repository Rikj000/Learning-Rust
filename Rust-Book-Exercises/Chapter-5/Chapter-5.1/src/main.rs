// Define a normal struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Define tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Define unit-like struct
struct AlwaysEqual;

fn main() {
    // Initialize a new User struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    // Modify one of it's parameters + print
    user1.email = String::from("anotheremail@example.com");
    println!("User1:\n- Username: {}\n- Email: {}", user1.email, user1.username);

    // Initialize a new User struct with a helper function
    let mut user2 = build_user(
        String::from("wakawaka@example.com"), String::from("MrPacManHimself"));

    println!("User2:\n- Username: {}\n- Email: {}", user2.email, user2.username);

    // Initialize a new User struct based off another user struct
    let user3 = User { email: String::from("another@example.com"), ..user1 }; // user1 becomes invalid...
    println!("User3:\n- Username: {}\n- Email: {}", user3.email, user3.username);

    // Initialize tuple structs
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Initialize unit-like struct
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}