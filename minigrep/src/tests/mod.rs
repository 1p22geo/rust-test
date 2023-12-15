use crate::search::search;

#[test]
fn one_result() {
    let query = "duct";
    let contents = "
Rust.
Whatever it is,
it's not fast, not safe.
And definitely not productive.
        ";
    assert_eq!(
        vec!["And definitely not productive."],
        search(query, contents)
    )
}

#[test]
fn multi_result() {
    let query = "duct";
    let contents = "
Rust.
Whatever it is,
Use duct tape.
And definitely not productive.
        ";
    assert_eq!(vec!["Use duct tape.", "And definitely not productive."], search(query, contents))
}

#[test]
fn no_result() {
    let query = "multithreading";
    let contents = "
Rust.
Whatever it is,
don't use it.
it's a trap.
    ";
    assert_eq!(vec![] as Vec<&str>, search(query, contents))
}

#[test]
fn case_sensitive() {
    let query = "A";
    let contents = "a";
    assert_eq!(vec![] as Vec<&str>, search(query, contents))
}
