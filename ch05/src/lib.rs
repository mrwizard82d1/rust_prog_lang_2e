use std::fmt;

pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("active", &self.active)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("sign_in_count", &self.sign_in_count)
            .finish()
    }
}
