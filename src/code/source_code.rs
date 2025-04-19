use std::{env, fs, process};

pub struct SourceCode {
    text: String,
}

impl SourceCode {
    pub fn new(text: String) -> Self { Self { text } }

    pub fn line_index(&self, position: usize) -> usize { return self.text[..position].lines().count() - 1; }

    pub fn get_line(&self, index: usize) -> &str { return self.text.lines().nth(index).unwrap(); }

    pub fn line_start(&self, index: usize) -> usize { return self.text.lines().take(index).map(|line| line.len() + 1).sum(); }
}

pub fn read_sourcefile() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Astra Compiler:\nNot enough arguments, for help use \"cargo run -- -h\" or \"cargo run -- --help\"");
        process::exit(1);
    }

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        eprintln!("Astra Compiler:\nUsage: cargo run -- <filename>");
        process::exit(0);
    }

    let passed_arg = &args[1];

    if passed_arg.split('.').nth(1) != Some("as") {
        eprintln!("Astra Compiler:\nProvided file has invalid extension, expected AstraScript file with \".as\" extension");
        process::exit(1);
    }

    match fs::read_to_string(passed_arg) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Astra Compiler:\nError reading file \"{}\": {}", passed_arg, err);
            process::exit(1);
        }
    }
}