use std::fmt::Display;

pub fn run() {
    let status1 = Status::SUCCESS(200, "ohlala".to_string());
    let status2 = Status::FORBIDDEN(403);
    run_status(&status1);
    run_status(&status2);

    run_option(Some(50));
    run_option(Some("j'ai la dalle"));
}

fn run_option(arg: Option<impl Display>) {
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
