mod scanner;

fn main() {
    println!("Hello world!");
    let mut scanner = scanner::Scanner::from_script("(Hello>), <= World!;");
    let tokens = scanner.scan();

    println!("Compiled! Tokens {}", tokens.len());
}

fn error(line: u32, message: &str) {
    report(line, "", message);
}

fn report(line: u32, loc: &str, message: &str) {
    format!("[Line {}] Error {}: {}", line, loc, message);
}
