pub struct AccountCredentials {
    pub username: String,
    pub password: String,
}

impl AccountCredentials {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}
