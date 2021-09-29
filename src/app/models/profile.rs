#[allow(dead_code)] 
pub struct Profile {
    id: u32,
    user_id: u32,
    first_name: String,
    last_name: String,
}

impl Profile {

    // Constructor
    #[allow(dead_code)] 
    pub fn new(
        id: u32, 
        user_id: u32, 
        first_name: &str, 
        last_name: &str
    ) -> Profile {
        Profile {
            id: id,
            user_id: user_id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    #[allow(dead_code)] 
    pub fn get_user_id(&self) -> u32 {
        self.user_id
    }

    #[allow(dead_code)]
    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
