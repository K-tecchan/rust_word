mod cli;

use clap::Parser;
use cli::{Cli, Type};
use rand::distributions::{Alphanumeric, DistString};

fn main() {
    let args = Cli::parse();
    let password = Alphanumeric.sample_string(&mut rand::thread_rng(), args.len);
    // let password_symbols = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    match args.r#type {
        Type::Normal => println!("{}", password),
        Type::Lower => println!("{}", password.to_lowercase()),
        Type::Upper => println!("{}", password.to_uppercase()),
    }

    println!("{:?}", args);
}
