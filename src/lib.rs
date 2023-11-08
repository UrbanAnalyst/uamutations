//! This is a stand-alone crate which implements the mutation algorithm for [Urban
//! Analyst](https://urbananalyst.city). The algorithm mutates selected properties for one city to
//! become more like those of another selected city.

pub mod read_write_file;
pub mod vector_fns;

/// This is the main function, which reads data from two JSON files, calculates absolute and
/// relative differences between the two sets of data, and writes the results to an output file.
///
/// # Arguments
///
/// * `fname1` - Path to local JSON file with data which are to be mutated.
/// * `fname2` - Path to local JSON file with data of mutation target towards which first data are
/// to be mutated.
/// * `varname` - Name of variable in both `fname1` and `fname2` to be mutated.
/// * `nentries` - The number of entries to be read from the JSON files.
/// * `outfilename` - Path to local output file.
///
/// # Process
///
/// 1. Reads the variable specified by `varname` from the files `fname1` and `fname2`.
/// 2. Calculates the absolute and relative differences between the two sets of data.
/// 3. Orders the relative differences in descending order.
/// 4. Writes the original data, the differences, and the ordering index to `outfilename`.
///
/// # Panics
///
/// This function will panic if the input files cannot be read, or if the output file cannot be written.
pub fn uamutate(fname1: &str, fname2: &str, varname: &str, nentries: usize, outfilename: &str) {
    let (index1, values1) = read_write_file::readfile(fname1, varname, nentries);
    let (index2, values2) = read_write_file::readfile(fname2, varname, nentries);

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

    read_write_file::write_file(&write_data, &ord_index, outfilename);
}
