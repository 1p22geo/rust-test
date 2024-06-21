use std::fs;

use crate::search::file_search;
use crate::search::str_search;

#[test]
fn test_str_search() {
    assert_eq!(
        str_search(
            &String::from("jeja.pl"),
            &String::from("jak ja k*rwa kocham jeja.pl\nnie no naprawde\n\nfr\njeja to życie\njeja.pl to miłość")
        ),
        vec!["jak ja k*rwa kocham jeja.pl", "jeja.pl to miłość"]
    );
}

#[test]
fn test_file_search() {
    match fs::write(
        "/tmp/minigrep-testing",
        "jak ja k*rwa kocham jeja.pl\nnie no naprawde\n\nfr\njeja to życie\njeja.pl to miłość",
    ) {
        Ok(_) => {}
        Err(e) => panic!("{e}\nfailed to open /tmp/minigrep-testing, make sure you use linux :3\ntests dont work on w*ndows"),
    }
    assert_eq!(
        file_search(
            &String::from("jeja.pl"),
            &String::from("/tmp/minigrep-testing")
        ),
        vec!["jak ja k*rwa kocham jeja.pl", "jeja.pl to miłość"]
    );
    match fs::remove_file("/tmp/minigrep-testing") {
        Ok(_) => {}
        Err(e) => panic!("{e}\nfailed to delete /tmp/minigrep-testing after running tests, make sure you use linux :3\ntests dont work on w*ndows"),
    }
}
