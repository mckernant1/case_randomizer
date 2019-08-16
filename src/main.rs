extern crate rand;

#[macro_use]
extern crate clap;

use rand::Rng;
use std::io::Read;
use clap::App;
use clap::Arg;

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
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


    loop {
        let mut buf = String::new();
        let bytes = std::io::stdin().read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }
        if alt {
            print!("{}", alternate_case(buf))
        } else {
            print!("{}", randomize_case(buf, freq_val));
        }
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
