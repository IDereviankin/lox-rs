mod scanner;

use scanner::Scanner;

fn read_line() -> String {
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).unwrap();
    result.trim().to_string()
}

fn run_file<T: AsRef<std::path::Path>>(path: T) {
    let program = String::from_utf8(std::fs::read(path).expect("Cannot read given file"))
        .expect("Wrong file encoding");
    run(program);
}

fn run_prompt() {
    loop {
        print!("> ");
        let line = read_line();
        if line == "" {
            break;
        };
        run(line);
    }
}

fn run<T: AsRef<str>>(source: T) {
    let s = source.as_ref();
    let scanner = Scanner::new(s);
    let tokens = scanner.scan_tokens();
    tokens.iter().for_each(|token| println!("{}", token));
}

fn main() {
    if let Some(filename) = std::env::args().nth(1) {
        run_file(filename);
    } else {
        run_prompt();
    }
}
