use std::fs;
#[derive(PartialEq, Eq, Debug)]
pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn read_from_env(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
        //mut - because iterating is mutable op
        args.next();

        let query = args.next().ok_or("Can't find query arg".to_string())?;

        let file_path = args.next().ok_or("Can't find file_path arg".to_string())?;

        let ignore_case = match args.next() {
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
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn read_file(file_path: &String) -> Result<String, String> {
    fs::read_to_string(file_path).map_err(|err| err.to_string())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_ignore_case<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query_ignore_case = query.to_lowercase();

    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_ignore_case))
        .collect()
}
