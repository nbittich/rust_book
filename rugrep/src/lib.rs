use crate::search::grep::Line;

pub mod argument;
pub mod search;


pub fn run(arguments: Vec<String>) -> Result<Vec<Line>, String> {
    let parsed_args = argument::parser::parse_args(&arguments);
    let mut search = String::new();
    let mut file_name = String::new();
    let mut poem = String::new();

    match parsed_args.query {
        Some(q) => search.push_str(&q),
        None => return Err("No query parameter found".to_string())
    };

    match parsed_args.file_name {
        Some(q) => file_name.push_str(&q),
        None => return Err("No file name parameter found".to_string())
    };

    match search::read::read_file(&file_name) {
        Ok(q) => poem.push_str(&q),
        Err(e) => return Err(format!("File could not be open: {}", e).to_string())
    };

    Ok(search::grep::grep(&poem, &search, &parsed_args.case_sensitive))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_run_no_query_parameter() {
        let arguments:Vec<String> = vec![];
        if let Err(err) = super::run(arguments) {
            assert_eq!("No query parameter found".to_string(), err);
        }else {
            panic!("shouldn't happen");
        }
    }

    #[test]
    fn test_run_no_file_name_parameter() {
        let arguments:Vec<String> = vec!["query=xxx".to_string()];
        if let Err(err) = super::run(arguments) {
            assert_eq!("No file name parameter found".to_string(), err);
        }else {
            panic!("shouldn't happen");
        }
    }

    #[test]
    fn test_run_no_file() {
        let arguments:Vec<String> = vec!["query=xxx".to_string(), "filename=xxx".to_string()];
        if let Err(err) = super::run(arguments) {
            assert!(err.contains("File could not be open"));
        }else {
            panic!("shouldn't happen");
        }
    }

    #[test]
    fn test_run_success() {
        let arguments:Vec<String> = vec!["query=nobody".to_string(), "filename=poem.txt".to_string()];
        if let Ok(result) = super::run(arguments) {
            assert_eq!(2, result.len());
            assert_eq!(1, result[0].index);
            assert_eq!("I’m nobody! Who are you?", result[0].line);
            assert_eq!(2, result[1].index);
            assert_eq!("Are you nobody, too?", result[1].line);
        }else {
            panic!("shouldn't happen");
        }
    }
}
