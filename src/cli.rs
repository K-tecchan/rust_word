use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "rw", bin_name = "rw", author, version, about)]
pub struct Cli {
    #[arg(default_value_t = 10)]
    pub len: usize,
    #[arg(value_enum, default_value_t = Type::Normal)]
    pub r#type: Type,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Type {
    Normal,
    Lower,
    Upper,
}
