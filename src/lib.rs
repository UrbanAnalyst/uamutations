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
/// * `varextra` - Extra variables to be considered in the mutation.
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
/// 2. dists: The relative degree by which each should be mutated.
///
/// # Panics
///
/// This function will panic if the input files cannot be read, or if the output file cannot be written.
pub fn uamutate(fname1: &str, fname2: &str, varname: &str, varextra: Vec<char>, nentries: usize, outfilename: &str) {
    let (index1, values1, _groups1) = read_write_file::readfile(fname1, varname, nentries);
    let (_index2, values2, _groups2) = read_write_file::readfile(fname2, varname, nentries);

    // The values are then sorted in in increasing order, and the indices map back to the original
    // order. The following line then calculates successive differences between the two sets of
    // values, where `false` is for the `absolute` parameter, so that differences are calculated
    // relative to values1.
    let dists_sorted = vector_fns::calculate_dists(&values1, &values2, false);
    // Then map those dists back onto the original order of `values1`:
    let dists: Vec<_> = index1.iter().map(|&i| dists_sorted[i]).collect();
    // let values: Vec<_> = index1.iter().map(|&i| values1[i]).collect();
    let groups: Vec<_> = index1.iter().map(|&i| _groups1[i]).collect();

    // Then aggregate 'dists' within 'groups', first by direct aggregation along with counts of
    // numbers of values aggregated within each group.
    let max_group = *groups.iter().max().unwrap();
    let mut counts = vec![0u32; max_group + 1];
    let mut sums = vec![0f64; max_group + 1];

    for (i, &group) in groups.iter().enumerate() {
        counts[group] += 1;
        sums[group] += dists[i];
    }

    // Then convert sums to mean values by dividing by counts:
    for (sum, count) in sums.iter_mut().zip(&counts) {
        *sum = if *count != 0 {
            *sum / *count as f64
        } else {
            0.0
        };
    }

    read_write_file::write_file(&sums, outfilename);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_uamutate() {
        // Define the input parameters for the function
        let filename1 = "./test_resources/dat1.json";
        let filename2 = "./test_resources/dat2.json";
        let varname = "bike_index";
        let varextra: [char; 0] = [];
        let nentries = 10;
        let outfilename = "/tmp/test_output.txt";

        // Call the function with the test parameters
        uamutate(filename1, filename2, varname, varextra.to_vec(), nentries, outfilename);

        // Check that the output file exists
        assert!(Path::new(outfilename).exists());

        // Open the file in read-only mode, returns `io::Result<File>`
        let file = fs::File::open(outfilename).expect("unable to open file");
        let reader = BufReader::new(file);

        // Read all lines into a vector
        let lines: Vec<_> = reader
            .lines()
            .collect::<Result<_, _>>()
            .expect("unable to read lines");

        // Check that the header contains the expected columns
        let header = &lines[0];
        assert!(header.contains("mutation"));
    }
}
