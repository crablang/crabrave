
fn main() {
    const TAPESIZE: usize = 69000;
    let mut data = [0u8; TAPESIZE];
    let mut ptr = 0;
data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;while data[ptr] != 0 {ptr += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;while data[ptr] != 0 {ptr += 1;data[ptr] += 1;data[ptr] += 1;ptr += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;ptr += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;ptr += 1;data[ptr] += 1;ptr -= 1;ptr -= 1;ptr -= 1;ptr -= 1;data[ptr] -= 1;}ptr += 1;data[ptr] += 1;ptr += 1;data[ptr] += 1;ptr += 1;data[ptr] -= 1;ptr += 1;ptr += 1;data[ptr] += 1;while data[ptr] != 0 {ptr -= 1;}ptr -= 1;data[ptr] -= 1;}ptr += 1;ptr += 1;print!("{}", data[ptr] as char);ptr += 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;print!("{}", data[ptr] as char);data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;print!("{}", data[ptr] as char);print!("{}", data[ptr] as char);data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;print!("{}", data[ptr] as char);ptr += 1;ptr += 1;print!("{}", data[ptr] as char);ptr -= 1;data[ptr] -= 1;print!("{}", data[ptr] as char);ptr -= 1;print!("{}", data[ptr] as char);data[ptr] += 1;data[ptr] += 1;data[ptr] += 1;print!("{}", data[ptr] as char);data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;print!("{}", data[ptr] as char);data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;data[ptr] -= 1;print!("{}", data[ptr] as char);ptr += 1;ptr += 1;data[ptr] += 1;print!("{}", data[ptr] as char);ptr += 1;data[ptr] += 1;data[ptr] += 1;print!("{}", data[ptr] as char);}