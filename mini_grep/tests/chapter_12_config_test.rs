use mini_grep::*;

#[test]
fn should_create_config() {
    let args = vec![
        "String1".to_string(),
        "String2".to_string(),
        "String3".to_string(),
        "IGNORE_CASE".to_string(),
    ];

    assert_eq!(
        Config::read_from_env(args.into_iter()).unwrap(),
        Config {
            file_path: "String3".into(),
            query: "String2".into(),
            ignore_case: true,
        }
    )
}

#[test]
fn should_search_with_ignored_case_correctly() {
    let query = "ABRA";
    let content = "\
CADA
abra
BRA";

    assert_eq!(search_ignore_case(query, content), vec!["abra"])
}
