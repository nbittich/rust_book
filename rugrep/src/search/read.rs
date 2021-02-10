use std::fs;

pub fn read_file(file_name: &String) -> std::io::Result<String> {
    fs::read_to_string(file_name)
}

#[cfg(test)]
mod test{
    use std::io::ErrorKind;

    #[test]
    fn test_read_file_not_found() {
        let result = super::read_file(&"blabla".to_string());
        match result {
            Ok(_str) => panic!("should not work"),
            Err(e) => assert_eq!(ErrorKind::NotFound, e.kind())
        }
    }

    #[test]
    fn test_read_file_found() {
        let result = super::read_file(&"poem.txt".to_string());
        match result {
            Ok(poem) => assert!(poem.contains("How public, like a frog")),
            Err(_e) => panic!("should not panic")
        }
    }
}
