//! This is a stand-alone crate which implements the mutation algorithm for [Urban
//! Analyst](https://urbananalyst.city). The algorithm mutates selected properties for one city to
//! become more like those of another selected city.

use std::fs::File;
use std::io::BufReader;

extern crate uamutations;
pub mod mlr;
pub mod read_write_file;
pub mod utils;

const NENTRIES: usize = 10000;
// const NENTRIES: usize = 1000;

// const FNAME1: &str = "berlin.json";
// const FNAME2: &str = "paris.json";
const FNAME1: &str = "/data/mega/code/repos/UrbanAnalyst/CityDataPrivate/berlin/dataraw.json";
const FNAME2: &str = "/data/mega/code/repos/UrbanAnalyst/CityDataPrivate/paris/dataraw.json";
// const FNAME1: &str = "./test_resources/dat1.json";
// const FNAME2: &str = "./test_resources/dat2.json";
// const VARNAME: &str = "bike_index";
const VARNAME: &str = "school_dist";
const OUTFILENAME: &str = "output.txt";

/// Entry point for the Urban Analyst mutation algorithm.
///
/// This exists only to locally call and run the library.
fn main() {
    // let varextra = vec!["natural".to_string(), "social_index".to_string()];
    // let varextra: Vec<String> = Vec::new();
    let varextra: Vec<String> = vec![
        "times_rel".to_string(),
        "times_abs".to_string(),
        "intervals".to_string(),
        "transport".to_string(),
        "school_dist".to_string(),
    ];

    let varsall: Vec<String> = vec![VARNAME.to_string()];
    let varsall = [varsall, varextra].concat();

    let file1 = File::open(FNAME1).unwrap();
    let reader1 = BufReader::new(file1);
    let file2 = File::open(FNAME2).unwrap();
    let reader2 = BufReader::new(file2);

    let sums = uamutations::uamutate(reader1, reader2, &varsall, NENTRIES);

    read_write_file::write_file(&sums, OUTFILENAME);
}
