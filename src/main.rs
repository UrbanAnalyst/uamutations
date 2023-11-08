mod read_write_file;
mod vector_fns;

const NENTRIES: usize = 1000;

const FNAME1: &str = "./test_resources/dat1.json";
const FNAME2: &str = "./test_resources/dat2.json";
const VARNAME: &str = "bike_index";
const OUTFILENAME: &str = "output.txt";

/// Entry point for the Urban Analyst mutation algorithm.
///
/// This function reads data from two JSON files, calculates absolute and relative differences
/// between the two sets of data, and writes the results to an output file.
///
/// # Files
///
/// * `FNAME1` and `FNAME2` - Paths to the input JSON files.
/// * `OUTFILENAME` - Path to the output file.
///
/// # Variables
///
/// * `VARNAME` - The name of the variable to be read from the JSON files.
/// * `NENTRIES` - The number of entries to be read from the JSON files.
///
/// # Process
///
/// 1. Reads the variable specified by `VARNAME` from the files `FNAME1` and `FNAME2`.
/// 2. Calculates the absolute and relative differences between the two sets of data.
/// 3. Orders the relative differences in descending order.
/// 4. Writes the original data, the differences, and the ordering index to `OUTFILENAME`.
///
/// # Panics
///
/// This function will panic if the input files cannot be read, or if the output file cannot be written.

fn main() {
    let (index1, values1) = read_write_file::readfile(FNAME1, VARNAME, NENTRIES);
    let (index2, values2) = read_write_file::readfile(FNAME2, VARNAME, NENTRIES);

    let diffs_abs = vector_fns::calculate_diffs(&values1, &values2, true);
    let diffs_rel = vector_fns::calculate_diffs(&values1, &values2, false);

    let ord_index = vector_fns::get_ordering_index(&diffs_rel, true); // true for is_abs

    // That creates the following 7 vectors of equal length:
    // 1. values1: Ordered vector of values which are to be changed = "from" values
    // 2. index1: Index mapping back to original order of 'values1'
    // 3. values2: Ordered vector of values to be mutated towards = "to" values
    // 4. index2: Index mapping back to original order of 'values2'
    // 5. diffs_abs: Vector of absolute differences between 'values1' and 'values2'
    // 6. diffs_rel: Vector of relative differences between 'values1' and 'values2'
    // 7. ord_index: Index mapping order of 'diffs_rel' back onto original order of 'values1'.

    let write_data = read_write_file::WriteData {
        values1,
        values2,
        diffs_abs,
        diffs_rel,
        index1,
        index2,
    };

    read_write_file::write_file(&write_data, &ord_index, OUTFILENAME);
}
