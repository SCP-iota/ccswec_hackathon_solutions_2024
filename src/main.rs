use std::{fs::read_to_string, io::stdin};
use clap::{builder::Str, Parser, Subcommand};

mod cmd01_palindrome_index;
mod cmd02_sparse_arrays;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    subcmd: Commands
}

#[derive(Subcommand)]
enum Commands {
    PalindromeIndex,
    SparseArrays
}

#[derive(Debug)]
enum HackathonError {
    InputReadError,
    FormatError,
    PalindromeIndexError(cmd01_palindrome_index::PalindromeIndexError)
}

impl std::fmt::Display for HackathonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HackathonError::InputReadError => write!(f, "Unable to read input file!"),
            HackathonError::FormatError => write!(f, "Unable to parse input!"),
            HackathonError::PalindromeIndexError(e) => write!(f, "{}", e)
        }
    }
}

impl std::error::Error for HackathonError {}

impl From<std::io::Error> for HackathonError {
    fn from(_: std::io::Error) -> Self {
        HackathonError::InputReadError
    }
}

impl From<std::num::ParseIntError> for HackathonError {
    fn from(_: std::num::ParseIntError) -> Self {
        HackathonError::FormatError
    }
}

impl From<cmd01_palindrome_index::PalindromeIndexError> for HackathonError {
    fn from(e: cmd01_palindrome_index::PalindromeIndexError) -> Self {
        HackathonError::PalindromeIndexError(e)
    }
}

fn read_lines_group() -> Result<Vec<String>, HackathonError> {
    let mut nlines_str = String::new();
    stdin().read_line(&mut nlines_str)?;
    let nlines = nlines_str.trim().parse::<usize>()?;
    let mut lines = vec!();

    for _ in 0..nlines {
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        lines.push(line.trim().to_string());
    }

    Ok(lines)
}

fn main() -> Result<(), HackathonError> {
    let cmd = Cli::parse();

    match cmd.subcmd {
        Commands::PalindromeIndex => {
            let lines = read_lines_group()?;

            for line in lines {
                println!("{}", cmd01_palindrome_index::palindrome_index(&line)?);
            }

            Ok(())
        },
        Commands::SparseArrays => {
            let strings = read_lines_group()?;
            let queries = read_lines_group()?;

            for n in cmd02_sparse_arrays::matchingStrings(&strings, &queries) {
                println!("{}", n);
            }

            Ok(())
        }
    }
}
