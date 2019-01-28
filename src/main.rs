extern crate rand;
extern crate clap;

use rand::Rng;
use std::io::Read;
use clap::App;
use clap::Arg;

fn main() -> std::io::Result<()> {
    let matches = App::new("cAsE rAnDoMiZeR")
        .version("0.1")
        .author("Tom M. tmeaglei@gmail.com")
        .about("\
        \nRandomizes the case of the input string from standard in\
        \nStrings are automatically given lower case")
        .arg(Arg::with_name("frequency")
            .short("f")
            .long("frequency")
            .value_name("float between 0 and 1")
            .help("sets the frequency of capital letters")
            .takes_value(true))
        .get_matches();

    let freq = matches.value_of("frequency").unwrap_or("default.conf").parse::<f64>().unwrap_or(0.5);
    dbg!(freq);

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    print!("{}", randomize_case(buf, freq));
    println!();
    Ok(())
}

fn randomize_case(s: String, freq: f64) -> String {
    let temp = s.chars().map(|s: char| {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(freq) {
            return s.to_ascii_uppercase();
        } else {
            return s.to_ascii_lowercase();
        }
    }).collect();

    return temp;
}