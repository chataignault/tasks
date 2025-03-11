use polars::prelude::*;
use polars_core::prelude::*;
use polars_io::csv::read::CsvReadOptions;
use polars_io::csv::write::CsvWriterOptions;
use polars_io::prelude::*;
use std::fs::File;
use std::io;
use std::path::Path;

fn main() {
    // CLI user interface to :
    // add new def to dictionary
    // query defs from dictionary (sample at random)
    // save the dictionary when leaving in csv file
    let mut file: String = String::from("vocab.csv");

    let mut voc = DataFrame::default();

    if Path::new(&file.to_string()).exists() {
        voc = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(file.clone().into()))
            .unwrap()
            .finish()
            .unwrap();
        println!("Found current voc : len {}", voc.shape().0);
        println!("{}", voc.head(Some(10)));
    }

    loop {
        print!("Enter action (add, ask, quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "add" => {}
            "ask" => {}
            "quit" => {
                let mut file = File::create(file).expect("could not create file");

                let _ = CsvWriter::new(&mut file)
                    .include_header(true)
                    .with_separator(b',')
                    .finish(&mut voc);
                break;
            }
            _ => println!("Unknown command. Please try again."),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_add_description() {}

    #[test]
    fn test_initialise_df() {}
}
