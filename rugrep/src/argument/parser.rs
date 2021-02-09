use std::collections::HashMap;

const ARG_QUERY: &str = "query=";
const ARG_FILE_NAME: &str = "filename=";

pub fn parse_args(arguments: &Vec<String>) -> HashMap<KnownArgument, &str> {
    let mut parsed_args: HashMap<KnownArgument, &str> = HashMap::new();
    if let Some(query) =map_arg(arguments, ARG_QUERY) {
        parsed_args.entry(KnownArgument::Query).or_insert(query);
    }
    if let Some(query) = map_arg(arguments, ARG_FILE_NAME) {
        parsed_args.entry(KnownArgument::FileName).or_insert(query);
    }

    parsed_args
}

fn map_arg<'a>(arguments: &'a Vec<String>, split_regex: &'a str) -> Option<&'a str> {
    arguments.iter()
        .find(|a| { a.starts_with(split_regex) })
        .and_then(|a| {
            let split = a.split(split_regex);
            split.skip(1).next().filter(|s| { !s.is_empty() })
        })
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum KnownArgument {
    Query,
    FileName,
}
