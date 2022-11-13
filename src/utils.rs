use std::fs;

pub fn chars(path: String) -> Vec<char> {
    let result = fs::read(path).expect("Fuck off :))");

    result.into_iter().map(|c| c as char).collect()
}