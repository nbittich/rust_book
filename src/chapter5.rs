use std::fmt::{Display, Formatter, Debug};

pub fn run() {
    let mut user: User = User {
        age: 32,
        firstname: String::from("Nordine"),
        lastname: String::from("Nordine"),
        active: true,
        email: String::from("nordine@localhost")
    };
    user.age +=1;
    println!("{}", user);
    let user_move = new_user(user, 32, String::from("Henry"), String::from("Toubon"));
    println!("{}", user_move);

    let tuple_struct: TupleStruct = TupleStruct(String::from("result"), String::from("ok"));
    println!("{:?}", tuple_struct); // debug trait usage single line
    println!("{:#?}", tuple_struct); // debug trait usage multi lines
    println!("email: {}", user_move.get_email());
    println!("Majority check: {}", user_move.check_age());
    println!("{}", User::new_instance(13, "momo".to_string(),"hamed".to_string(),true, "momo@hamed.com".to_string()))
}

fn new_user(from: User,age:u32, firstname: String, lastname: String) -> User {
    User {
        age,
        firstname,
        lastname,
        ..from
    }
}

#[derive(Debug)]
struct TupleStruct(String, String);

struct User {
    age: u32,
    firstname: String,
    lastname: String,
    active: bool,
    email: String
}

impl User {
    fn get_email(&self) -> &str {
        &self.email[..]
    }
    fn check_age(&self) -> &str {
        if self.age >= 18 {"You can drive a car"} else {"You can watch disney junior"}
    }
    fn new_instance(age:u32, firstname:String, lastname: String, active:bool, email: String) -> User {
        User {
            age,
            firstname,
            lastname,
            active,
            email
        }
    }
}


impl Display for User{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "age: {}, firstname: {}, lastname: {}, active: {}, email: {}",self.age, self.firstname, self.lastname, self.active, self.email)
    }
}