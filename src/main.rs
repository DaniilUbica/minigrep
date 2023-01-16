use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content;

    let config = match Config::new(args) {
        Ok(smth) => smth,
        Err(err) => {eprintln!("Error parsing arguments: {}", err); panic!("Restart app to try again!")}
    };

    if let Err(e) = minigrep::read_file(&config) {
        eprintln!("Error in app: {}", e);
        panic!("Restart app to try again!");
    }
    else {
        content = minigrep::read_file(&config).unwrap();
    }

    println!("\nSearching {} in {}, where the text is: \n{}", &config.query, &config.filename, content);

    for i in minigrep::search(&config.query, &content, config.case_sensitive) {
        println!("Word {} founded in line: {}", &config.query, i);
    }
}

