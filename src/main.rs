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
        .arg(Arg::with_name("alternate")
            .short("a")
            .long("alternate")
            .help("non-random alternation starting with the first letter lowercase")
            .takes_value(false))
        .get_matches();

    let freq_is_present = matches.is_present("frequency");

    let alt = matches.is_present("alternate");

    let mut freq_val = 0.5;
    if freq_is_present {
        let freq = matches.value_of("frequency").unwrap_or("default.conf").parse::<f64>();
        freq_val = match freq {
            Ok(x) => x,
            Err(_e) => {
                panic!("The frequency must be a float between 0 and 1");
            }
        };
        if freq_val > 1.0 || freq_val < 0.0 {
            panic!("The frequency must be a float between 0 and 1");
        }
    }

    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    if alt {
        print!("{}", alternate_case(buf))
    } else {
        print!("{}", randomize_case(buf, freq_val));
    }

    println!();
    Ok(())
}

fn randomize_case(s: String, freq: f64) -> String {
    let mut rng = rand::thread_rng();
    s.chars().map(|c: char| {
        if rng.gen_bool(freq) {
            return c.to_ascii_uppercase();
        } else {
            return c.to_ascii_lowercase();
        }
    }).collect()
}

fn alternate_case(s: String) -> String {
    let mut alternate = true;
    s.chars().map(move |c| {
        if alternate {
            alternate = false;
            return c.to_ascii_lowercase();
        } else {
            alternate = true;
            return c.to_ascii_uppercase();
        }
    }).collect()
}