use testrust::account::entity::{User, Role};
use testrust::account::service::random_password;
use testrust::account::service::random_role;

fn main() {
    let user: User = User::create_user("Nordine".to_string(),"Bittich".to_string(), 32, Role::ADMIN, "strong_password".to_string());
    println!("Password matches: {}", user.check_password("ouloulou"));
    println!("Password matches: {}", user.check_password("strong_password"));
    println!("Password random: {}", random_password());
    println!("Role random: {:?}", random_role());
}
