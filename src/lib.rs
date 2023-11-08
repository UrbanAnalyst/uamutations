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
/// The following seven vectors of equal length are written to the output file:
/// 1. values: The original values of 'varname' from 'fname1'.
/// 2. diffs: The relative degree by which each should be mutated.
///
/// # Panics
///
/// This function will panic if the input files cannot be read, or if the output file cannot be written.
pub fn uamutate(fname1: &str, fname2: &str, varname: &str, nentries: usize, outfilename: &str) {
    let (index1, values1) = read_write_file::readfile(fname1, varname, nentries);
    let (_index2, values2) = read_write_file::readfile(fname2, varname, nentries);
    // The values are then sorted in in increasing order, and the indices map back to the original
    // order. The following line then calculates successive differences between the two sets of
    // values, where `false` is for the `absolute` parameter, so that differences are calculated
    // relative to values1.
    let diffs_sorted = vector_fns::calculate_diffs(&values1, &values2, false);
    // Then map those diffs back onto the original order of `values1`:
    let diffs: Vec<_> = index1.iter().map(|&i| diffs_sorted[i]).collect();
    let values: Vec<_> = index1.iter().map(|&i| values1[i]).collect();

    let write_data = read_write_file::WriteData { values, diffs };

    read_write_file::write_file(&write_data, outfilename);
}
