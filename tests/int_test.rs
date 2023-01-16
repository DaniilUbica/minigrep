use minigrep::Config;

#[test]
fn config_new_check_failed() {
    let args:Vec<String> = vec![];
    let b: bool;

    match Config::new(args) {
        Ok(_) => b = true,
        Err(_) => b = false,
    };
    assert!(!b);
}

#[test]
fn config_new_check_passed() {
    let args:Vec<String> = vec!["qwe".to_string(), "we".to_string(), "poem.txt".to_string()];
    let b: bool;

    match Config::new(args) {
        Ok(_) => b = true,
        Err(_) => b = false,
    };
    assert!(b);
}

#[test]
fn read_file_test_failed() {
    let b: bool;
    let c = Config {
        query: "Some query".to_string(),
        filename: "Some filename".to_string(),
        case_sensitive: false,
    };

    if let Err(_) = minigrep::read_file(&c) {
        b = false;
    }
    else {
        b = true;
    }

    assert!(!b);
}

#[test]
fn read_file_test_passed() -> Result<(), String>{
    let c = Config {
        query: "Some query".to_string(),
        filename: "poem.txt".to_string(),
        case_sensitive: false,
    };

    if let Err(_) = minigrep::read_file(&c) {
        Err(String::from("Cannot read file"))
    }
    else {
        Ok(())
    }
}