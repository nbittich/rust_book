pub use std::fmt::{Display, Formatter, Debug};

pub mod account {
    pub mod entity {
        #[derive(Debug)]
        pub enum Role {
            ANONYMOUS, USER, ADMIN
        }
        #[derive(Debug)]
        pub struct User {
            pub firstname: String,
            pub lastname: String,
            pub role: Role,
            password: String,
            pub age: u32,
        }
        impl User {
            pub fn check_password(&self, password_provided: &str) -> bool {
                  password_provided.eq(&self.password[..])
            }
            pub fn create_user (firstname: String, lastname: String, age:u32, role: Role, password: String) -> User {
                User {
                    firstname,
                    lastname,
                    role,
                    password,
                    age
                }
            }
        }
    }
    pub mod service {
        use rand::{distributions::Alphanumeric,Rng};

        use super::entity::{Role};

        pub fn random_password () -> String {
            rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect()
        }

        pub fn random_role () -> Role {
            match rand::thread_rng().gen_bool(0.5) {
                true => Role::ADMIN,
                false => Role::USER
            }
        }
    }
}
