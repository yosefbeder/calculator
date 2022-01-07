use super::calculator::calculate;
use std::io;

fn read() -> Result<String, String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(err) => Err(format!("[REPL]: {}", err)),
    }
}

fn eval(input: &str) -> Result<f64, String> {
    calculate(input)
}

fn print(output: Result<f64, String>) {
    match output {
        Ok(n) => println!("{}\n", n),
        Err(err) => println!("{}\n", err),
    }
}

pub fn run() {
    loop {
        let input = read().unwrap();
        let output = eval(&input);
        print(output);
    }
}
