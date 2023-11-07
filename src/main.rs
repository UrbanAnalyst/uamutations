use std::fs::File;
use std::io::Write;

mod readfile;
mod vector_fns;

const NENTRIES: usize = 1000;
const FNAME1: &str = "dat1.json";
const FNAME2: &str = "dat2.json";
const VARNAME: &str = "bike_index";

fn main() {
    let (index1, values1) = readfile::readfile(FNAME1, VARNAME, NENTRIES);
    let (index2, values2) = readfile::readfile(FNAME2, VARNAME, NENTRIES);

    let diffs_abs = vector_fns::calculate_diffs(&values1, &values2, true);
    let diffs_rel = vector_fns::calculate_diffs(&values1, &values2, false);

    let ord_index = vector_fns::get_ordering_index(&diffs_rel, true); // true for is_abs

    let mut file = File::create("output.txt").expect("Unable to create file");

    for ((((((number1, number2), dabs), drel), i1), i2), oi) in values1
        .iter()
        .zip(values2.iter())
        .zip(diffs_abs.iter())
        .zip(diffs_rel.iter())
        .zip(index1.iter())
        .zip(index2.iter())
        .zip(ord_index.iter())
    {
        write!(
            file,
            "{}, {}, {}, {}, {}, {}, {}\n",
            number1, number2, dabs, drel, i1, i2, oi
        )
        .expect("Unable to write to file");
    }
}
