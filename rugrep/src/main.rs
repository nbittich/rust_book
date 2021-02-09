use crate::argument::parser::KnownArgument;

pub mod argument;
pub mod search;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let parsed_args = argument::parser::parse_args(&arguments);
    let search = parsed_args.get(&KnownArgument::Query).unwrap();
    let file_name = parsed_args.get(&KnownArgument::FileName).unwrap();

    let poem = search::read::read_file(file_name).unwrap();
    for l in search::grep::grep(&poem, search) {
        println!("{}: {}", l.index, l.line);
    }
}
