use crate::MyResult;
use clap::{Arg, Command};

pub struct Config {
    lifespan: i32,
}

impl Config {
    pub fn lifespan(&self) -> i32 {
        self.lifespan
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("lifer")
        .version("0.1.0")
        .author("Mikkel Productions")
        .about("A CLI game of Conway's Game of Life")
        .arg(
            Arg::new("lifespan")
                .short('l')
                .long("lifespan")
                .value_name("LIFESPAN")
                .default_value("60")
                .help("Lifespan in seconds")
                .value_parser(clap::value_parser!(i32)),
        )
        .get_matches();

    return Ok(Config {
        lifespan: matches.get_one::<i32>("lifespan").unwrap().to_owned(),
    });
}
