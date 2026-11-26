use std::io::{Write, stdin, stdout};

use crate::{snake::snake_main, tetris::tetris_main};

mod common;
mod flappy_bird;
mod snake;
mod tetris;

fn main() {
    let standard_in = stdin();
    print!("\x1b[?1049h");
    loop {
        print!("\x1b[2J\x1b[H");
        println!("What game do you want to play?");
        println!("1) Tetris");
        println!("2) Flappy bird");
        println!("3) Snake");
        print!(": ");
        let _ = stdout().flush();

        let mut selection = String::new();
        standard_in
            .read_line(&mut selection)
            .expect("Error while reading line");
        match selection.to_lowercase().trim() {
            "q" | "exit" | "quit" => break,
            "1" | "tetris" => tetris_main(),
            "2" | "flappy bird" => {}
            "3" | "snake" => snake_main(),
            _ => println!("\"{selection}\" is not a valid option, \"exit\" to quit!"),
        }
    }
    // General clean up
    print!("\x1b[?1049l");
    let _ = stdout().flush();
}
