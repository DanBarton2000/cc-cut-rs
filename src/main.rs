use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;

fn main() {
    let config = Config::build().expect("Failed to create config");
    cut(config).expect("Failed to perform cut");
}

struct Config {
    fields: Vec<usize>,
    delimiter: char,
    reader: Box<dyn BufRead>
}

impl Config {
    fn build() -> std::io::Result<Config> {
        let args = Args::parse();
        let reader = build_reader(args.path)?;
        let delimiter = args.delimiter.unwrap_or('\t');

        let mut fields: Vec<usize> = vec![];

        let split: Vec<_> = if args.fields.contains(',') {
            args.fields.split(',').collect()
        } else {
            args.fields.split(' ').collect()
        };

        if split.is_empty() {
            panic!("Failed to parse the fields argument: {}", args.fields);
        }

        for split in split {
            fields.push(split.parse::<usize>().expect("Failed to parse to usize") - 1);
        }

        Ok(Config {
            fields,
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
            match File::open(&path) {
                Ok(file) => {
                    Ok(Box::new(BufReader::new(file)))
                }
                Err(error) => {
                    if path.to_str() == Some("-") {
                        return Ok(Box::new(BufReader::new(stdin())));
                    }
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
        let mut output = String::new();

        for (index, field) in config.fields.iter().enumerate() {
            if *field < split.len() {
                output.push_str(split[*field]);
            }
            if index < config.fields.len() - 1 {
                output.push(config.delimiter);
            }
        }

        if !output.is_empty() {
            println!("{output}");
        }

        line.clear();
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short='f', long="fields")]
    fields: String,
    #[clap(short='d', long="delimiter")]
    delimiter: Option<char>,
    path: Option<PathBuf>
}