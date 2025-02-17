use mini_grep::*;
use std::env;
use std::env::Args;

//to run:
//cargo run -p mini_grep -- YOU poem.txt IGNORE_CASE
fn main() -> Result<(), String> {
    let args: Args = env::args(); //it's iterator

    let config = Config::read_from_env(args)?; //pass ownership due to we need to iterate

    let content = read_file(&config.file_path)?;

    let result = if config.ignore_case {
        search_ignore_case(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    if result.is_empty() {
        println!("Lines not found")
    } else {
        println!("The result lines: \n {}", result.join("\n"))
    }

    Ok(())
}
