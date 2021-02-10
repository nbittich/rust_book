pub fn grep(file_str: &String, search: &String, case_sensitive: &bool) -> Vec<Line> {
    let mut result: Vec<Line> = vec![];
    let split = file_str.split("\n");
    let mut index = 1;
    let search_sensitivity =|s: &str| {if *case_sensitive {s.to_string()} else {s.to_lowercase()}};
    for line in split {
        if search_sensitivity(line).contains(&search_sensitivity(search)) {
            result.push(Line {
                index,
                line: line.to_string(),
            });
        }
        index += 1;
    }
    result
}

pub struct Line {
    pub index: u32,
    pub line: String,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_grep () {
        let str =  String::from("Another day at bikini bottom\nI love money - Kaptain Krabs\nOuloulou");
        let result = super::grep(&str, &"Krabs".to_string(), &true);
        assert_eq!(1, result.len());
        assert_eq!(2, result[0].index);
        assert_eq!("I love money - Kaptain Krabs", result[0].line);
        let result = super::grep(&str, &"Not_found".to_string(), &true);
        assert!(result.is_empty());
    }
}
