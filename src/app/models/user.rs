#[allow(dead_code)] 
pub struct User {
    id: u32,
    email: String
}

impl User {

    #[allow(dead_code)] 
    pub fn new(id: u32, email: &str) -> User {
        User {
            id: id,
            email: email.to_string()
        }
    }

    #[allow(dead_code)] 
    pub fn get_id(&self) -> u32 {
        self.id
    }
}
