fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    match rugrep::run(arguments) {
        Ok(lines) => {
            for l in lines {
                println!("{}: {}", l.index, l.line);
            }
        },
        Err(e) =>{
            eprintln!("Error: {}",e);
            std::process::exit(-1);
        }
    }
}
