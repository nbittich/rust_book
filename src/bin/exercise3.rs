use std::collections::{BTreeMap};

fn main() {

    let mut my_company: BTreeMap<String, Vec<String>> = BTreeMap::new();

    loop {
        let mut command = String::new();
        println!("Enter command");
        std::io::stdin().read_line(&mut command).expect("Error while reading");
        if command.starts_with("exit") {
            break;
        }
        let sanitized_command = command.replace("\n","");
        let mut split = sanitized_command.split(" ");

        match split.next() {
            Some("ADD") | Some("add") => {
                let name = split.next().expect("Name not present!");
                let _to = split.next().filter(|s| {s.eq_ignore_ascii_case("to")}).expect("To not present!");
                let department = split.next().expect("Department not present!").to_uppercase();
                let departments = my_company.entry(department.to_lowercase()).or_insert(vec![]);
                departments.push(name.to_lowercase())
            },
            Some("BY_DEPARTMENT") | Some("by_department") => {
                let department = split.next().expect("Department not present!").to_lowercase();
                let sanitized_department = department.replace("\n","");
                let mut employees = my_company.get(sanitized_department.as_str()).expect("Department not found").to_vec();
                employees.sort();
                for e in employees {
                    println!("{}", e)
                }
            },
            Some("BY_COMPANY") | Some("by_company") => {
                for  key in my_company.keys() {
                    let mut v = my_company[key].to_vec();
                    v.sort();
                    for e in v {
                        println!("{} -> {}", key, e)
                    }
                }

            },
            _ => {
                println!("Unknown command");
            }
        }

    }

}
