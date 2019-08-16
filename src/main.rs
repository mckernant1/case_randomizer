extern crate rand;

#[macro_use]
extern crate clap;

use rand::Rng;
use std::io::Read;
use clap::App;
use clap::Arg;
use std::fs::File;

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

    if matches.is_present("FILES") {
        let file_list: Vec<_> = matches.values_of("FILES").unwrap().collect();
        read_from_file(freq_val, alt, file_list)
    } else {
        read_from_stdin(freq_val, alt)
    }

    println!();
    Ok(())
}

fn read_from_file(freq_val: f64, alt: bool, file_list: Vec<&str>) {
    for file in file_list {
        let mut buf = String::new();
        File::open(file).unwrap().read_to_string(&mut buf).unwrap();
        print_string(buf, freq_val, alt)
    }
}

fn read_from_stdin(freq_val: f64, alt: bool) {
    loop {
        let mut buf = String::new();
        let bytes = std::io::stdin().read_line(&mut buf).unwrap();
        if bytes == 0 {
            break;
        }
        print_string(buf, freq_val, alt)
    }
}

fn print_string(buf: String, freq_val: f64, alt: bool){
    if alt {
        print!("{}", alternate_case(buf))
    } else {
        print!("{}", randomize_case(buf, freq_val));
    }
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
