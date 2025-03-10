use polars::prelude::*;
use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs::File;
use std::io;

fn main() {
    // CLI user interface to :
    // add new def to dictionary
    // query defs from dictionary (sample at random)
    // save the dictionary when leaving in csv file
    let mut file: String = String::from("vocab.csv");

    loop {
        print!("Enter action (add, ask, quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut df = DataFrame::default();

        match input {
            "add" => {}
            "ask" => {}
            "quit" => {
                let mut file = File::create(file).expect("could not create file");

                CsvWriter::new(&mut file)
                    .include_header(true)
                    .with_separator(b',')
                    .finish(&mut df);
                break;
            }
            _ => println!("Unknown command. Please try again."),
        }
    }
}
