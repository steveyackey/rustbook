use::std::collections::HashMap;
pub fn new_department_list() -> HashMap<String, Vec<String>> {
    let depts: HashMap<String, Vec<String>> = HashMap::new();
    depts
}

pub fn parse_command(departments: &mut HashMap<String, Vec<String>>, command: String) {
    let words: Vec<&str> = command.split_whitespace().collect();
    departments.entry(String::from(words[3])).or_insert(Vec::new()).push(String::from(words[1]));
}
