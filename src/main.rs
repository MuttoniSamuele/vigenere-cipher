mod vigenere;

use std::io::{self, Write};

#[derive(PartialEq)]
enum Action {
    Cipher,
    Decipher,
}

fn main() {
    println!("# Vigenere Cipher #");
    println!("\nSelect an action:");
    let is_cipher = ask_action() == Action::Cipher;
    println!(
        "\nMessage to {}",
        if is_cipher { "cipher" } else { "decipher" }
    );
    let message = ask_input();
    println!("\nKey");
    let key = ask_input();
    let output = if is_cipher {
        vigenere::vigenere_cipher(&message, &key)
    } else {
        vigenere::vigenere_decipher(&message, &key)
    };
    println!("\n{}", output);
}

fn ask_input() -> String {
    print!("> ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error while reading input");
    input
}

fn ask_action() -> Action {
    println!("1) Cipher");
    println!("2) Decipher");
    loop {
        match ask_input().trim() {
            "1" => {
                break Action::Cipher;
            }
            "2" => {
                break Action::Decipher;
            }
            _ => {
                println!("Enter either 1 or 2");
            }
        }
    }
}
