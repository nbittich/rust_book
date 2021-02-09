pub fn grep(file_str:&String, search: &str ) -> Vec<Line>{
    let mut result: Vec<Line> = vec![];
    let split = file_str.split("\n");
    let mut index = 1;
    for line in split {
        if line.contains(search) {
            result.push(Line{
                index,
                line: line.to_string()
            });
        }
        index += 1;
    }
    result
}

pub struct Line {
    pub index: u32,
    pub line: String
}
