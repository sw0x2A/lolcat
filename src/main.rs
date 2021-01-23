use clap::{value_t, App, Arg};
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args = App::new("lolcat")
        .version("0.1")
        .about("Prints in rainbow colours")
        .arg(
            Arg::with_name("freq")
                .help("frequency")
                .short("f")
                .long("freq")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("spread")
                .help("spread")
                .short("s")
                .long("spread")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("input")
                .index(1)
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let freq = value_t!(args, "freq", f64).unwrap_or(0.2);
    let spread = value_t!(args, "spread", f64).unwrap_or(2.5);
    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, freq, spread);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, freq, spread);
    }
}

fn process_lines<T: BufRead + Sized>(reader: T, freq: f64, spread: f64) {
    let seed = rand::thread_rng().gen_range(0, 256) as f64;
    let get_rainbow_color = |i: f64| {
        use std::f64::consts::PI;
        let red = ((freq * i + 0.0).sin() * 127.0 + 128.0) as u8;
        let green = ((freq * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;
        let blue = ((freq * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;
        return format!("\x1b[38;2;{};{};{}m", red, green, blue);
    };
    for (i, line_) in reader.lines().enumerate() {
        let line = line_.unwrap();
        for (j, char) in line.chars().enumerate() {
            print!(
                "{}{}",
                get_rainbow_color((seed + i as f64 + j as f64) / spread),
                char
            )
        }
        println!("\x1b[0m")
    }
}
