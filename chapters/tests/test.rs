use chapters::Message;

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

//cargo tests by default run all tests in parallel (you can use <cargo tests -- --tests-threads=1>)
//for specifying tests name - <cargo tests one_hundred>

//if we write tests in the same file as code you need:
//#[cfg(tests)]
// mod tests {} - to exclude this module from result build file.

//so tests folder if for integration tests. They won't run if any unit tests failed
//cargo tests --tests integration_test
