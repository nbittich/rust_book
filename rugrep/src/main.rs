use std::fs;
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    println!("{:?}",arguments);
    let search = &arguments[1];
    let file_name = &arguments[2];
    println!("{}",search);
    println!("{}",file_name);
    let poem = fs::read_to_string(file_name).unwrap();
    let split = poem.split("\n");
    let mut index = 1;
    for line in split {
        if line.contains(search) {
            println!("{}: {}",index, line);
        }
        index+=1;
    }

}
