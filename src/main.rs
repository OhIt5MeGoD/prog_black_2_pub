// Authors: Neshesh Rai, Oliver Veal, Jed Nicholson
use rust_games::games::{connect_four};
use std::io;

fn main() {
    let mut exit: bool = false;
    while !exit {
        println!("=== Game Menu ===");
        println!("1. Connect 4");
        println!("2. Snake");
        println!("3. Blackjack");
        println!("4. Exit");

        let choice = get_choice();
        if choice == 1 {
            println!("Selected Connect 4");
            println!();
            connect_four::play_connect4();
        }
        if choice == 2 {
            println!("Selected Snake");
            println!();
        }
        if choice == 3 {
            println!("Selected Blackjack");
            println!();
        }
        if choice == 4 {
            exit = true;
            println!("Bye!")
        }
    }
}

fn get_choice() -> usize {
    let mut choice_str = String::new();
    io::stdin().read_line(&mut choice_str).expect("Failed to read line");
    println!("");
    let choice: usize = match choice_str.trim().parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    return choice;
}