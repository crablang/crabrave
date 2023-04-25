use std::{
    env,
    fs::{self, canonicalize},
    process::Command,
};

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
use std::io::{stdin, Read};
fn main() {
    const SIZE: usize = %SIZE%;
    let mut data = [0u8; SIZE];
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
            Token::Input => crab.push_str("data[ptr] = stdin().bytes().next().unwrap().unwrap();"),
            Token::LoopStart => crab.push_str("while data[ptr] != 0 {"),
            Token::LoopEnd => crab.push_str("}"),
        }
    }
    crab.push_str("}");

    crab
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file> [tapesize]", args[0]);
        return;
    }

    let input_file = &args[1];

    let brainfuck = fs::read_to_string(input_file).expect("Couldn't read the brainfuck!");
    let tokens = lexer(&brainfuck);

    let mut tape_size = "69420";
    if args.len() > 2 && args[2].as_str().parse::<usize>().is_ok() {
        tape_size = &args[2];
    }

    let crab = translate_to_crab(&tokens).replace("%SIZE%", tape_size);
    let basename = std::path::Path::new(input_file)
        .file_stem()
        .to_owned()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let rs_filename = basename.clone() + ".rs";
    fs::write(&rs_filename, crab).expect("error making crab file!");

    let output = Command::new("crabc")
        .arg(&*rs_filename)
        .arg("-o")
        .arg(&basename)
        .output()
        .expect("couldnt compile the transpiled crab to an executable!");

    if !output.status.success() {
        eprintln!("ERRRRRR: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        // remove the crab file
        let _ = fs::remove_file(rs_filename);
        println!(
            "successfully compiled to {}",
            canonicalize(basename)
                .unwrap()
                .to_string_lossy()
                .to_string()
        );
    }
}
