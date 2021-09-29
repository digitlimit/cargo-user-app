// This declaration will look for a file named `app.rs` or `app/mod.rs` and will
// insert its contents inside a module named `my` under this scope
mod app;
use app::models::user::User;

fn main() {
    let user: User = User::new(
        1, 
        "frankemeks77@yahoo.com"
    );

    print!("{}", user.get_id());
}
