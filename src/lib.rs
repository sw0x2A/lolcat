use clap::{value_t, App, Arg};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    if config.input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, config);
    } else {
        let f = File::open(config.input.clone())?;
        let reader = BufReader::new(f);
        process_lines(reader, config);
    }

    Ok(())
}

pub struct Config {
    pub freq: f64,
    pub seed: f64,
    pub spread: f64,
    pub input: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args = App::new("lolcat")
            .version("0.2")
            .about(
                "Concatenate FILE(s), or standard input, to standard output.
With no FILE, or when FILE is -, read standard input.",
            )
            .arg(
                Arg::with_name("freq")
                    .help("Rainbow frequency")
                    .short("F")
                    .long("freq")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("seed")
                    .help("Rainbow seed, 0 = random")
                    .short("S")
                    .long("seed")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("spread")
                    .help("Rainbow spread")
                    .short("p")
                    .long("spread")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("input")
                    .index(1)
                    .help("FILE or STDIN")
                    .takes_value(true)
                    .required(false),
            )
            .get_matches();

        let freq = value_t!(args, "freq", f64).unwrap_or(0.2);
        let seed = value_t!(args, "seed", f64).unwrap_or(0.0);
        let spread = value_t!(args, "spread", f64).unwrap_or(2.5);
        let input = args.value_of("input").unwrap_or("-").to_string();

        Ok(Config {
            freq,
            seed,
            spread,
            input,
        })
    }
}

pub fn process_lines<T: BufRead + Sized>(reader: T, config: &Config) {
    let seed = match config.seed as u8 {
        0 => rand::thread_rng().gen_range(0, 256) as f64,
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
