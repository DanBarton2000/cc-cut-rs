use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;

fn main() {
    let config = Config::build().expect("Failed to create config");
    cut(config).expect("Failed to perform cut");
}

struct Config {
    fields: usize,
    delimiter: char,
    reader: Box<dyn BufRead>
}

impl Config {
    fn build() -> std::io::Result<Config> {
        let args = Args::parse();
        let reader = build_reader(args.path)?;
        let delimiter = args.delimiter.unwrap_or_else(|| '\t');
        Ok(Config {
            fields: args.fields - 1,
            delimiter,
            reader
        })
    }
}

fn build_reader(path: Option<PathBuf>) -> std::io::Result<Box<dyn BufRead>> {
    match path {
        None => {
            Ok(Box::new(BufReader::new(stdin())))
        }
        Some(path) => {
            match File::open(path) {
                Ok(file) => {
                    Ok(Box::new(BufReader::new(file)))
                }
                Err(error) => {
                    Err(error)
                }
            }
        }
    }
}

fn cut(mut config: Config) -> std::io::Result<()> {
    let mut line = String::new();
    while config.reader.read_line(&mut line)? > 0 {
        let split: Vec<&str> = line.split(config.delimiter).collect();

        if config.fields < split.len() {
            println!("{}", split[config.fields]);
        }

        line.clear();
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short='f', long="fields")]
    fields: usize,
    #[clap(short='d', long="delimiter")]
    delimiter: Option<char>,
    path: Option<PathBuf>
}