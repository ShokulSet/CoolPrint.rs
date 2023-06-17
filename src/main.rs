use std::env;
use std::{thread, time};
use std::char;

fn sequence_print(word : &str, start : u16, end : u16, time : u64) {
    let lenght: u16 = word.len() as u16;
    let mut printed: &str;
    for i in 0..lenght { 
        for j in start..end {
            printed = &word[0..i as usize];
            println!("\u{1b}[2J\u{1b}[1;1H{}{}", printed, char::from_u32(j as u32).unwrap());
            thread::sleep(time::Duration::from_millis(time));
            if j == word.as_bytes()[i as usize] as u16 {
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("\u{1b}[31mError : Invalid arguments.\u{1b}[39m");
        std::process::exit(1);
    }
    let word = &args[1];
    let lang = &args[2];
    if lang == "en" {
        sequence_print(word, 65, 123, 10);
    } else if lang == "th" {
        sequence_print(word, 3584, 3711, 10);
    }
    else {
        println!("\u{1b}[31mError : Unknown language.\u{1b}[39m");
        std::process::exit(1);
    }
}
