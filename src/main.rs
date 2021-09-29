// This declaration will look for a file named `app.rs` or `app/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod app;
use app::models::user::User;
use app::models::profile::Profile;

fn main() {
    let user: User = User::new(
        1, 
        "frankemeks77@yahoo.com"
    );

    let profile: Profile = Profile::new(
        1,
        1,
        "Franke",
        "Mekel",
    );

    println!("User:get_id() --> {}", user.get_id());
    println!("Profile::get_full_name() --> {}", profile.get_full_name());
}
