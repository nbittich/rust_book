use testrust::account::entity::{User, Role};
use testrust::account::store::service::{random_password, add_session_to_store, find_user_by_lastname};
use testrust::account::store::service::random_role;
use testrust::account::store::service::add_user_to_store;
use testrust::account::store::Storage;

fn main() {
    let user: User = User::create_user("Nordine".to_string(),"Bittich".to_string(), 32, Role::ADMIN, "strong_password".to_string());
    println!("Password matches: {}", user.check_password("ouloulou"));
    println!("Password matches: {}", user.check_password("strong_password"));
    println!("Password random: {}", random_password());
    println!("Role random: {:?}", random_role());

    let mut storage: Storage = Storage(Vec::new());
    add_user_to_store(&mut storage, user);
    add_session_to_store(&mut storage, "ouloulou".to_string());
    let user_found = find_user_by_lastname(storage, "Bittich");
    if let Some(usr) = user_found {
        println!("it's a mess but it works => {:?}", usr);
    }
}
