//! This is a stand-alone crate which implements the mutation algorithm for [Urban
//! Analyst](https://urbananalyst.city). The algorithm mutates selected properties for one city to
//! become more like those of another selected city.

use ndarray::{s, Array2};

pub mod mlr;
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
pub fn uamutate(
    fname1: &str,
    fname2: &str,
    varname: &str,
    varextra: Vec<String>,
    nentries: usize,
    outfilename: &str,
) {
    let varsall: Vec<String> = vec![varname.to_string()];
    let varsall = [varsall, varextra].concat();
    let (mut values1, groups1) = read_write_file::readfile(fname1, &varsall, nentries);
    let (values2, _groups2) = read_write_file::readfile(fname2, &varsall, nentries);

    // Then adjust `values1` by removing its dependence on varextra, and replacing with the
    // dependnece of values2 on same variables.
    adj_for_beta(&mut values1, &values2);

    // Then calculate successive differences between the two sets of values, where `false` is for
    // the `absolute` parameter, so that differences are calculated relative to values1. These are
    // then the distances by which `values1` need to be moved in the first dimension only to match
    // the closest equivalent values of `values21`.
    let dists = vector_fns::calculate_dists(&values1, &values2, false);
    // Then aggregate these relative distances within the groups defined in the original `groups1`
    // vector, first by direct aggregation along with counts of numbers of values aggregated within
    // each group.
    let groups: Vec<_> = groups1;
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

/// Adjusts the first row of `values1` based on the multi-linear regression coefficients of the
/// remaining rows of `values1` against `values2`.
///
/// This effectivly removes the dependence of the first row of `values1` by on all other
/// variables/rows, and replaces it with the dependence of `values2` on the same variables.
///
/// # Arguments
///
/// * `values1` - A 2D array where the first row is the variable to be adjusted and the remaining
/// rows are the other variables.
/// * `values2` - A 2D array with the same structure as `values1`, used to calculate the MLR
/// coefficients for adjustment.
///
/// * Example
/// let mut v1 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [2.1, 3.2, 4.1, 5.2, 5.9]];
/// let v1_orig = v1.clone();
/// let v2 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [3.1, 4.3, 5.3, 6.5, 7.3]];
/// adj_for_beta(&mut v1, &v2);
/// assert_ne!(v1, v1_orig, "v1 should differ from v1_orig");
/// assert_eq!(
///     v1.slice(s![1.., ..]),
///     v1_orig.slice(s![1.., ..]),
///     "Only the first row of v1 should be different"
/// );
fn adj_for_beta(values1: &mut Array2<f64>, values2: &Array2<f64>) {
    // Calculate MLR regression coefficients between first variables and all others:
    let beta1 = mlr::mlr_beta(values1);
    let beta2 = mlr::mlr_beta(values2);
    // Then adjust `values1` by removing its dependence on those variable, and replacing with the
    // dependnece of values2 on same variables:
    let mut result = ndarray::Array1::zeros(values1.ncols());
    for i in 0..values1.ncols() {
        let b1 = ndarray::Array1::from(beta1.clone());
        let b2 = ndarray::Array1::from(beta2.clone());
        let values_slice = values1.slice(s![1.., i]).to_owned();
        let product = &values_slice * (1.0 + &b2 - &b1);
        result[i] = product.sum();
    }
    values1.row_mut(0).assign(&result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use std::fs;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
    fn test_adj_for_beta() {
        let mut v1 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [2.1, 3.2, 4.1, 5.2, 5.9]];
        let v1_orig = v1.clone();
        let v2 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [3.1, 4.3, 5.3, 6.5, 7.3]];
        adj_for_beta(&mut v1, &v2);
        assert_ne!(
            v1, v1_orig,
            "v1 should be different from v1_orig after adj_for_beta"
        );
        assert_eq!(
            v1.slice(s![1.., ..]),
            v1_orig.slice(s![1.., ..]),
            "Only the first row of v1 should be different"
        );
    }

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
