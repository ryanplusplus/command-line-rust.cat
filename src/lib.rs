use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::{error::Error, io::BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type CatResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> CatResult<Config> {
    let matches = App::new("cat")
        .version("0.1.0")
        .author("ryanplusplus")
        .about("cat, but Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number the output lines, starting at 1")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number the non-blank output lines, starting at 1")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn file_reader(filename: &str) -> CatResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> CatResult<()> {
    for file in config.files.iter() {
        match file_reader(file) {
            Ok(reader) => {
                let mut line_number = 1;
                for line in reader.lines() {
                    let line = line?;
                    if config.number_lines || (config.number_nonblank_lines && !line.is_empty()) {
                        println!("{:>6}\t{}", line_number, line);
                        line_number += 1;
                    } else {
                        println!("{}", line);
                    }
                }
            }

            Err(e) => {
                eprintln!("{}: {}", file, e);
            }
        }
    }

    Ok(())
}
