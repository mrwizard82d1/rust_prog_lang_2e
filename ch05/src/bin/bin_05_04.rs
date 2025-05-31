use ch05::User;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("someone@example.com"), 
                           String::from("someusername123"));
    println!("user1: {:?}", user1);
}