use std::fmt::Display;

pub fn run() {
    let status1 = Status::SUCCESS(200, "ohlala".to_string());
    let status2 = Status::FORBIDDEN(403);
    let status3 = Status::ERROR(500, "server".to_string());

    run_status(&status1);
    run_status(&status2);

    run_option(Some("50"));
    run_option(Some("j'ai la dalle"));
    run_option(None);

    let (status_code,message) = status3.extract();

    if let Some(msg) = message {
        println!("{}: {}", status_code, msg);
    } else {
        println!("{}", status_code);
    }
}

fn run_option(arg: Option<&str>) {
    if let Some(val)  = arg {
        println!("{}", val);
    }
}

fn run_status(status: &Status) {
    match status {
        Status::SUCCESS(status_code, message) => println!("{} : {}", status_code, message),
        Status::FORBIDDEN(status_code) => println!("{}: {}", status_code, "et j'sers même le weekend"),
        _ => ()
    }
}

enum Status {
    SUCCESS(u32,String),
    FORBIDDEN(u32),
    UNAUTHORIZED(u32),
    ACCEPTED(u32),
    ERROR(u32, String)
}

impl Status {
    fn extract (&self) -> (&u32, Option<&String>) {
        match self {
            Status::SUCCESS(status_code, message) => (status_code,Some(message)),
            Status::ERROR(status_code, message) => (status_code,Some(message)),
            Status::FORBIDDEN(status_code) => (status_code, None),
            Status::UNAUTHORIZED(status_code) => (status_code,None),
            Status::ACCEPTED(status_code) => (status_code,None),
        }
    }
}

fn main() {
    run();
}
