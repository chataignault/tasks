use polars::prelude::*;
// use polars_core::prelude::*;
use polars_io::csv::read::CsvReadOptions;
// use polars_io::csv::write::CsvWriterOptions;
// use polars_io::prelude::*;
use rand::rng;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io;
use std::path::Path;

fn load_data(file: &String) -> DataFrame {
    let mut voc = DataFrame::new(vec![
        Column::new("Word".into(), &Vec::<String>::new()),
        Column::new("Description".into(), &Vec::<String>::new()),
    ])
    .unwrap();

    if Path::new(&file.to_string()).exists() {
        voc = open_csv(file).unwrap();
        println!("Found current voc : len {}", voc.shape().0);
        println!("{}", voc.head(Some(10)));
    }
    voc
}

fn open_csv(file: &String) -> Result<DataFrame, PolarsError> {
    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(file.clone().into()))
        .unwrap()
        .finish()
}

fn main() {
    // CLI user interface to :
    // add new def to dictionary
    // query defs from dictionary (sample at random)
    // save the dictionary when leaving in csv file
    let file: String = String::from("vocab.csv");

    let mut voc = load_data(&file);

    loop {
        println!("Enter action (add, ask / (word, des), quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let total_rows = voc.height();
        let mut rng = rng();

        match input {
            "add" => {
                let mut word = String::new();
                let mut des = String::new();

                println!("Enter word:");
                io::stdin().read_line(&mut word).unwrap();
                let word = word.trim();

                println!("Enter description:");
                io::stdin().read_line(&mut des).unwrap();
                let des = des.trim();
                let new_row = DataFrame::new(vec![
                    Column::new("Word".into(), [word]),
                    Column::new("Description".into(), [des]),
                ])
                .unwrap();

                // Append it to the existing DataFrame
                voc = voc.vstack(&new_row).unwrap();
            }
            "ask" => {
                let mut indices: Vec<usize> = (0..total_rows).collect();
                indices.shuffle(&mut rng);
                let random_indices: Vec<usize> = indices.into_iter().take(10).collect();
                let mask = (0..total_rows)
                    .map(|i| random_indices.contains(&i))
                    .collect::<Vec<bool>>();
                let sample_df = voc
                    .filter(&BooleanChunked::new(
                        PlSmallStr::from_static("filter"),
                        &mask,
                    ))
                    .unwrap();
                println!("{}", sample_df);
            }
            "word" => {
                let mut indices: Vec<usize> = (0..total_rows).collect();
                indices.shuffle(&mut rng);
                let random_indices: Vec<usize> = indices.into_iter().take(10).collect();
                let mask = (0..total_rows)
                    .map(|i| random_indices.contains(&i))
                    .collect::<Vec<bool>>();
                let sample_df = voc
                    .filter(&BooleanChunked::new(
                        PlSmallStr::from_static("filter"),
                        &mask,
                    ))
                    .unwrap();
                println!(
                    "{}",
                    sample_df
                        .select([PlSmallStr::from_static("Word")])
                        .unwrap()
                        .clone()
                );
            }
            "des" => {
                let mut indices: Vec<usize> = (0..total_rows).collect();
                indices.shuffle(&mut rng);
                let random_indices: Vec<usize> = indices.into_iter().take(10).collect();
                let mask = (0..total_rows)
                    .map(|i| random_indices.contains(&i))
                    .collect::<Vec<bool>>();
                let sample_df = voc
                    .filter(&BooleanChunked::new(
                        PlSmallStr::from_static("filter"),
                        &mask,
                    ))
                    .unwrap();
                println!(
                    "{}",
                    sample_df
                        .select([PlSmallStr::from_static("Description")])
                        .unwrap()
                        .clone()
                );
            }
            "quit" => {
                let mut file = File::create(file).expect("could not create file");

                voc = voc
                    .sort(
                        [PlSmallStr::from_static("Word")],
                        SortMultipleOptions::default()
                            .with_order_descending_multi(vec![false])
                            .with_maintain_order(true),
                    )
                    .unwrap();

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

    use crate::*;

    #[test]
    fn test_open_csv() {
        assert!(open_csv(&"vocab.csv".to_string()).is_ok());
    }
}
