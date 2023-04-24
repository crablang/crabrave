use std::{env, fs, process::Command};

#[derive(Debug, PartialEq)]
enum Token {
    IncrementPointer,
    DecrementPointer,
    IncrementData,
    DecrementData,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

fn lexer(code: &str) -> Vec<Token> {
    code.chars()
        .filter_map(|c| match c {
            '🐙' | '>' => Some(Token::IncrementPointer), // >
            '🦑' | '<' => Some(Token::DecrementPointer), // <
            '🦀' | '+' => Some(Token::IncrementData),    // +
            '🦞' | '-' => Some(Token::DecrementData),    // -
            '🐚' | '.' => Some(Token::Output),           // .
            '🦐' | ',' => Some(Token::Input),            // ,
            '🐋' | '[' => Some(Token::LoopStart),        // [
            '🐬' | ']' => Some(Token::LoopEnd),          // ]
            _ => None,
        })
        .collect()
}

fn translate_to_crab(tokens: &[Token]) -> String {
    let mut crab = r#"
fn main() {
    const TAPESIZE: usize = 69000;
    let mut data = [0u8; TAPESIZE];
    let mut ptr = 0;
"#
    .to_string();
    for token in tokens {
        match token {
            Token::IncrementPointer => crab.push_str("ptr += 1;"),
            Token::DecrementPointer => crab.push_str("ptr -= 1;"),
            Token::IncrementData => crab.push_str("data[ptr] = data[ptr].wrapping_add(1);"),
            Token::DecrementData => crab.push_str("data[ptr] = data[ptr].wrapping_sub(1);"),
            Token::Output => crab.push_str("print!(\"{}\", data[ptr] as char);"),
            Token::Input => {
                crab.push_str("data[ptr] = std::io::stdin().bytes().next().unwrap().unwrap();")
            }
            Token::LoopStart => crab.push_str("while data[ptr] != 0 {"),
            Token::LoopEnd => crab.push_str("}"),
        }
    }
    crab.push_str("}");

    crab
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return;
    }

    let brainfuck = fs::read_to_string(&args[1]).expect("Couldn't read the brainfuck!");
    let tokens = lexer(&brainfuck);
    let crab = translate_to_crab(&tokens);
    fs::write("transpiled.rs", &crab).expect("error making crab file!");

    let output = Command::new("crabc")
        .arg("transpiled.rs")
        .arg("-o")
        .arg("compiled")
        .output()
        .expect("couldnt compile the transpiled crab to an executable!");

    if !output.status.success() {
        eprintln!("ERRRRRR: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("we made it!");
    }
}
