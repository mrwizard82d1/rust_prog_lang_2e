use ch05::User;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };
    
    // Because `username` and `email` are of type `String`, the expressions, 
    // `username: user1.username` and `email: user1.email` involve a **move**
    // and not a copy.
    // println!("user1 = {:?}", user1);
    println!("user2 = {:?}", user2);
}