//! This is a stand-alone crate which implements the mutation algorithm for [Urban
//! Analyst](https://urbananalyst.city). The algorithm mutates selected properties for one city to
//! become more like those of another selected city.

use std::fs::File;
use std::io::Write;

extern crate uamutations;
pub mod read_write_file;
pub mod vector_order;

const NENTRIES: usize = 2000;

// const FNAME1: &str = "berlin.json";
// const FNAME2: &str = "paris.json";
const FNAME1: &str = "./test_resources/dat1.json";
const FNAME2: &str = "./test_resources/dat2.json";
const VARNAME: &str = "bike_index";
const OUTFILENAME: &str = "output.txt";

fn profile_methods(nentries: usize) -> (String, String, String) {
    let varsall: Vec<String> = vec![VARNAME.to_string()];
    let varextra = vec!["natural".to_string()];
    let varsall = [varsall, varextra].concat();
    let (_index1, values1, _groups1) = read_write_file::readfile(FNAME1, &varsall, nentries);
    let (_index2, values2, _groups2) = read_write_file::readfile(FNAME2, &varsall, nentries);

    use std::time::Instant;

    let start = Instant::now();
    let _order_kd2 = vector_order::order_vectors_kd(&values1, &values2);
    let duration_kd = format!("{:.4}", start.elapsed().as_secs_f64());

    let start = Instant::now();
    let _order2 = vector_order::order_vectors(&values1, &values2);
    let duration_dists = format!("{:.4}", start.elapsed().as_secs_f64());

    let start = Instant::now();
    let _pca1 = vector_order::pca(&values1);
    let duration_pca = format!("{:.4}", start.elapsed().as_secs_f64());

    (duration_kd, duration_dists, duration_pca)
}

/// Entry point for the Urban Analyst mutation algorithm.
///
/// This exists only to locally call and run the library.
fn main() {
    let mut file = File::create("timings.csv").expect("Unable to create file");
    writeln!(file, "n, kd, dists, pca").expect("Unable to write to file");
    for n in 1..20 {
        // let nentries = n * 100;
        let nentries = n * 10;
        let times = profile_methods(nentries);
        writeln!(file, "{}, {}, {}, {}", nentries, times.0, times.1, times.2)
            .expect("Unable to write to file");
        println!("n = {}", nentries);
    }

    let varextra: Vec<String> = Vec::new();
    // let varextra = vec!["natural".to_string()];
    uamutations::uamutate(FNAME1, FNAME2, VARNAME, varextra, NENTRIES, OUTFILENAME);
}
