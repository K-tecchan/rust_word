mod cli;

use clap::Parser;
use rand::distributions::{Alphanumeric, Distribution};

use cli::{Cli, Type};

fn main() {
    let args = Cli::parse();
    let password_symbols = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let mut password = String::new();

    for _ in 0..args.len {
        if rand::random::<u8>() > 64 {
            password.push(Alphanumeric.sample(&mut rand::thread_rng()) as char);
        } else {
            password.push(
                password_symbols
                    .chars()
                    .nth(rand::random::<usize>() % password_symbols.len())
                    .unwrap(),
            );
        }
    }

    match args.r#type {
        Type::Normal => println!("{}", password),
        Type::Lower => println!("{}", password.to_lowercase()),
        Type::Upper => println!("{}", password.to_uppercase()),
    }

    println!("{:?}", args);
}
