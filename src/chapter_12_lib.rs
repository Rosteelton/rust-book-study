use std::fs;
#[derive(PartialEq, Eq, Debug)]
pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn read_from_env(env_vars: &Vec<String>) -> Result<Config, String> {
        if env_vars.len() < 3 {
            return Err("not enough arguments".to_string());
        }

        let query = env_vars.get(1).ok_or("Can't find query arg".to_string())?;

        let file_path = env_vars
            .get(2)
            .ok_or("Can't find file_path arg".to_string())?;

        let ignore_case = match env_vars.get(3) {
            Some(str) => {
                if str == "IGNORE_CASE" {
                    true
                } else {
                    return Err(format!("Incorrect option {}", str));
                }
            }
            None => false,
        };

        println!(
            "Searching for {} in file {}; with ignoring case = {}",
            query, file_path, ignore_case
        );

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })
    }
}

pub fn read_file(file_path: &String) -> Result<String, String> {
    fs::read_to_string(file_path).map_err(|err| err.to_string())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_ignore_case<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query_ignore_case = query.to_lowercase();
    let mut result: Vec<&str> = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&query_ignore_case) {
            result.push(line);
        }
    }
    result
}
