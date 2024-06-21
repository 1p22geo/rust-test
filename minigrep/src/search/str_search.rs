pub fn str_search(query: &String, content: &String) -> Vec<String> {
    let mut found_matches: Vec<String> = vec![];
    for line in content.split("\n") {
        if line.contains(query) {
            found_matches.push(line.to_owned())
        }
    }
    found_matches
}
