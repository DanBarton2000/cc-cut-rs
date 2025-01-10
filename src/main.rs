use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let reader = get_reader(args.path).expect("Failed to create reader");
    cut(reader, args.fields - 1).expect("Failed to perform cut");
}

fn get_reader(path: Option<PathBuf>) -> std::io::Result<Box<dyn BufRead>> {
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

fn cut(mut reader: Box<dyn BufRead>, fields: usize) -> std::io::Result<()> {
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let split: Vec<&str> = line.split('\t').collect();

        if fields < split.len() {
            println!("{}", split[fields]);
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
    path: Option<PathBuf>
}