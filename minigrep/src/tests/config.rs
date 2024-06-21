use crate::config::Config;

#[test]
fn test_config_values() {
    let config = Config {
        query: String::from("asdf"),
        files: vec![
            String::from("./file.txt"),
            String::from("./file2.txt"),
            String::from("./file3.txt"),
        ],
    };
    assert_eq!(config.files.len(), 3);
    assert_eq!(config.files[1], "./file2.txt");
    assert_eq!(config.query, "asdf");
}

#[test]
fn test_config_parse() {
    let config = Config::parse(vec![
        String::from("/usr/bin/minigrep"),
        String::from("pattern"),
        String::from("bshuffle.service"),
        String::from("pulseaudio.service"),
    ]);
    assert_eq!(config.files.len(), 2);
    assert_eq!(config.files[1], "pulseaudio.service");
    assert_eq!(config.query, "pattern");
}
