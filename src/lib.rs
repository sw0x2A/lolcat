use clap::{Command, Arg};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    for input in &config.inputs {
        if input == "-" {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, config);
        } else {
            let f = File::open(input)?;
            let reader = BufReader::new(f);
            process_lines(reader, config);
        }
    }
    Ok(())
}

pub struct Config {
    pub freq: f64,
    pub seed: f64,
    pub spread: f64,
    pub inputs: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args = Command::new("lolcat")
            .version("0.2")
            .about(
                "Concatenate FILE(s), or standard input, to standard output.\n\
                 With no FILE, or when FILE is -, read standard input.",
            )
            .arg(
                Arg::new("freq")
                    .help("Rainbow frequency")
                    .short('F')
                    .long("freq")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("seed")
                    .help("Rainbow seed, 0 = random")
                    .short('S')
                    .long("seed")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("spread")
                    .help("Rainbow spread")
                    .short('p')
                    .long("spread")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::new("inputs")
                    .multiple_occurrences(true)
                    .help("FILE or STDIN")
                    .takes_value(true)
                    .default_value("-")
                    .required(false),
            )
            .get_matches();

        let freq = args.value_of_t("freq").unwrap_or(0.2);
        let seed = args.value_of_t("seed").unwrap_or(0.0);
        let spread = args.value_of_t("spread").unwrap_or(2.5);
        let inputs = args
            .values_of("inputs")
            .unwrap()
            .map(String::from)
            .collect();

        Ok(Config {
            freq,
            seed,
            spread,
            inputs,
        })
    }
}

pub fn process_lines<T: BufRead + Sized>(reader: T, config: &Config) {
    let seed = match config.seed as u8 {
        0 => rand::thread_rng().gen_range(0.0..256.0),
        _ => config.seed,
    };
    let get_color = |i: f64| {
        use std::f64::consts::PI;
        let red = ((config.freq * i + 0.0).sin() * 127.0 + 128.0) as u8;
        let green = ((config.freq * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;
        let blue = ((config.freq * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;
        return format!("\x1b[38;2;{};{};{}m", red, green, blue);
    };
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        for (j, char) in line.chars().enumerate() {
            print!(
                "{}{}",
                get_color((seed + i as f64 + j as f64) / config.spread),
                char
            )
        }
        println!("\x1b[0m")
    }
}
