use rust_book_study::Message;

#[test]
fn check_is_quit() {
    assert!(Message::Quit.is_quit(),);
    assert_eq!(Message::Quit.is_quit(), true)
}

//#[should_panic] - for exception

//if we want to match specific exception:

// panic!(
//     "Guess value must be less than or equal to 100, got {}.",
//     value
// );

//and then specify substring of exception message

//#[should_panic(expected = "less than or equal to 100")]

//cargo test by default run all test in parallel (you can use <cargo test -- --test-threads=1>)
//for specifying test name - <cargo test one_hundred>

//if we write test in the same file as code you need:
//#[cfg(test)]
// mod tests {} - to exclude this module from result build file.

//so tests folder if for integration tests. They won't run if any unit test failed
//cargo test --test integration_test