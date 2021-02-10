use std::env;

const ARG_QUERY: &str = "query=";
const ARG_FILE_NAME: &str = "filename=";
const CASE_SENSITIVE: &str = "CASE_SENSITIVE";

pub fn parse_args(arguments: &Vec<String>) -> KnownArgument {
    let mut known_arg = KnownArgument::new();
    if let Some(query) = map_arg(arguments, ARG_QUERY) {
        known_arg.query = Some(query);
    }
    if let Some(file_name) = map_arg(arguments, ARG_FILE_NAME) {
        known_arg.file_name = Some(file_name);
    }
    known_arg.case_sensitive = env::var(CASE_SENSITIVE).is_err();

    known_arg
}

fn map_arg<'a>(arguments: &'a Vec<String>, split_regex: &'a str) -> Option<String> {
    arguments.iter()
        .find(|a| { a.starts_with(split_regex) })
        .and_then(|a| {
            let split = a.split(split_regex);
            split.skip(1).next().filter(|s| { !s.is_empty() })
            .map(|s| {s.to_string()})
        })
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct KnownArgument {
    pub query: Option<String>,
    pub file_name: Option<String>,
    pub case_sensitive: bool
}

impl KnownArgument {
    fn new() -> KnownArgument {
        return KnownArgument{
            query: None,
            file_name: None,
            case_sensitive: true
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn map_arg_test() {
        let split_regex = "[id]=";
        let arguments = vec![split_regex.to_string() + "55"];
        let result = super::map_arg(&arguments, split_regex);
        match result {
            None => panic!("Shouldn't happen"),
            Some(value) => assert_eq!("55".to_string(), value )
        }
    }

    #[test]
    fn parse_arg_test() {
        let arguments = vec![super::ARG_QUERY.to_string() + "query",super::ARG_FILE_NAME.to_string() + "filename"];
        let result = super::parse_args(&arguments);
        assert_eq!(result.file_name, Some("filename".to_string()));
        assert_eq!(result.query, Some("query".to_string()));
    }
}
