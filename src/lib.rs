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


        pub mod store {
            use super::entity::{User, Role};

            pub enum NoSqlEntry {
                USER(User),
                SESSION(String)
            }

            pub struct Storage(pub Vec<NoSqlEntry>);

            impl Storage {
                fn push(&mut self, entry: NoSqlEntry) {
                    self.0.push(entry);
                }

            }

            pub mod service {
                use rand::{distributions::Alphanumeric,Rng};

                use crate::account::store::{Role, NoSqlEntry, Storage};
                use crate::account::entity::User;

                pub fn random_password () -> String {
                    rand::thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect()
                }

                pub fn random_role () -> Role {
                    match rand::thread_rng().gen_bool(0.5) {
                        true => Role::ADMIN,
                        false => Role::USER
                    }
                }

                pub fn add_user_to_store(storage: &mut Storage, user: User)  {
                    storage.push(NoSqlEntry::USER(user));
                }

                pub fn add_session_to_store(storage:&mut Storage, session: String) {
                    storage.push(NoSqlEntry::SESSION(session));
                }

                pub fn find_user_by_lastname(storage:Storage, lastname: &str) -> Option<User> {
                    let mut resp: Option<User> = None;
                    for item in storage.0 {
                        match item {
                            NoSqlEntry::USER(user) => {
                                if user.lastname.eq(lastname) {
                                    resp = Some(user);
                                    break;
                                }
                            },
                            _ => ()
                        }
                    }
                    resp
                }

            }


        }


}
