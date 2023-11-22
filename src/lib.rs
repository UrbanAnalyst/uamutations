//! This is a stand-alone crate which implements the mutation algorithm for [Urban
//! Analyst](https://urbananalyst.city). The algorithm mutates selected properties for one city to
//! become more like those of another selected city.

pub mod calculate_dists;
pub mod mlr;
pub mod read_write_file;

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
pub fn uamutate(
    fname1: &str,
    fname2: &str,
    varname: &str,
    varextra: Vec<String>,
    nentries: usize,
    outfilename: &str,
) {
    let varsall: Vec<String> = vec![varname.to_string()];
    let num_varextra = varextra.len();
    let varsall = [varsall, varextra].concat();
    let (mut values1, groups1) = read_write_file::readfile(fname1, &varsall, nentries);
    let (mut values2, _groups2) = read_write_file::readfile(fname2, &varsall, nentries);

    // standardise inputs to same scales for each variables:
    mlr::standardise_arrays(&mut values1, &mut values2);
    // Then adjust `values1` by removing its dependence on varextra, and replacing with the
    // dependnece of values2 on same variables (but only if `varextra` are specified):
    if num_varextra > 0 {
        mlr::adj_for_beta(&mut values1, &values2);
    }

    // Then calculate successive differences between the two sets of values, where `false` is for
    // the `absolute` parameter, so that differences are calculated relative to values1. These are
    // then the distances by which `values1` need to be moved in the first dimension only to match
    // the closest equivalent values of `values21`.
    let dists = calculate_dists::calculate_dists(&values1, &values2, false);
    let sums = aggregate_to_groups(&dists, &groups1);

    read_write_file::write_file(&sums, outfilename);
}

/// Aggregate distances within the groups defined in the original `groups` vector.
///
/// # Arguments
///
/// * `dists` - A vector of distances between entries in `values1` and closest values in `values2`.
/// * `groups` - A vector of same length as `dists`, with 1-based indices of group numbers. There
/// will generally be far fewer unique groups as there are entries in `dists`.
fn aggregate_to_groups(dists: &[f64], groups: &[usize]) -> Vec<f64> {
    let groups_out: Vec<_> = groups.to_vec();
    let max_group = *groups_out.iter().max().unwrap();
    let mut counts = vec![0u32; max_group + 1];
    let mut sums = vec![0f64; max_group + 1];

    for (i, &group) in groups_out.iter().enumerate() {
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

    // First value of `sums` is junk because `groups` are 1-based R values:
    sums.remove(0);

    sums
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
        let varextra: Vec<String> = Vec::new();
        let nentries = 10;
        let outfilename = "/tmp/test_output.txt";

        // Call the function with the test parameters
        uamutate(
            filename1,
            filename2,
            varname,
            varextra,
            nentries,
            outfilename,
        );

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
