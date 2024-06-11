use std::fs;
use std::fs::File;
use std::io::{Error, Read};
use std::io::ErrorKind::NotFound;

fn main() {

    let file = File::open("text.txt").unwrap_or_else(|error| {
        if error.kind() == NotFound {
            File::create_new("text.txt").unwrap_or_else(|error| {
                panic!("Can't create file: {:?}", error)
            })
        } else {
            panic!("HZ");
        }
    });

    //Operations on Result:
    //unwrap_or_else - if we want to match on error
    //unwrap - if we want to panic
    //expect - if we want to panic and provide message

    //the same as fs::read_to_string("hello.txt")
    fn read_username_from_file() -> Result<String, Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        //примерно тоже самое как делать флэтмап в скале, если ошибка, то дальше не пойдем и вернем ее наверх
        //таким образом "?" - делает unwrap или propogate ошибки наверх
        //можно использовать там где есть реализация FromResidual
        Ok(username)
    }



}