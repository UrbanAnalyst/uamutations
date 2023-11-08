use itertools::Itertools;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

/// Reads a JSON file and returns a tuple of two vectors: one for the indices and one for the
/// values.
///
/// # Arguments
///
/// * `filename` - The path to the JSON file to be read.
/// * `varname` - The name of the variable to be read from the JSON file.
/// * `nentries` - The number of entries to be read from the JSON file.
///
/// # Panics
///
/// This function will panic if `nentries` is less than or equal to zero, or if the file cannot be
/// read.
///
/// # Returns
///
/// A tuple of two vectors:
/// * The first vector contains the indices of the sorted values.
/// * The second vector contains the sorted values.
///
/// # Example
///
/// ```
/// use uamutations::read_write_file::readfile;
/// let filename = "./test_resources/dat1.json";
/// let varname = "transport";
/// let nentries = 10;
/// let (index, values, groups) = readfile(filename, varname, nentries);
/// ```

pub fn readfile(
    filename: &str,
    varname: &str,
    nentries: usize,
) -> (Vec<usize>, Vec<f64>, Vec<usize>) {
    assert!(nentries > 0, "nentries must be greater than zero");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader).unwrap();

    let mut values = Vec::new();
    let mut city_group = Vec::new();
    let city_group_col = "index";

    let mut var_exists = false;

    if let Value::Array(array) = &json {
        for item in array {
            if values.len() >= nentries {
                break;
            }
            if let Value::Object(map) = item {
                if let Some(Value::Number(number)) = map.get(varname) {
                    var_exists = true;
                    if let Some(number) = number.as_f64() {
                        values.push(number);
                    }
                }
                if let Some(Value::Number(number)) = map.get(city_group_col) {
                    if let Some(number) = number.as_f64() {
                        city_group.push(number as usize);
                    }
                }
            }
        }
    }

    assert!(
        var_exists,
        "Variable {} does not exist in the JSON file",
        varname
    );

    let mut pairs: Vec<_> = values.into_iter().enumerate().collect();
    pairs.sort_unstable_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());

    let index: Vec<usize> = pairs.iter().map(|&(index, _)| index).collect();
    let values: Vec<f64> = pairs.iter().map(|&(_, value)| value).collect();

    let mut original_order: Vec<usize> = vec![0; values.len()];
    let mut groups: Vec<usize> = vec![0; values.len()];
    for (i, &idx) in index.iter().enumerate() {
        original_order[idx] = i;
        groups[idx] = city_group[i];
    }

    assert_eq!(
        index.len(),
        values.len(),
        "The lengths of index and values are not equal"
    );
    assert_eq!(
        original_order.len(),
        values.len(),
        "The lengths of index and values are not equal"
    );
    assert!(
        values.iter().tuple_windows().all(|(a, b)| a <= b),
        "values are not sorted"
    );

    (original_order, values, groups)
}

/// Writes the mean mutation values to a file.
///
/// # Arguments
///
/// * `sums` - Mutation values aggregated into city polygons.
/// * `filename` - The name of the file to which the data will be written.
///
/// # Panics
///
/// This function will panic if it fails to create or write to the file.
pub fn write_file(sums: &[f64], filename: &str) {
    let mut file = File::create(filename).expect("Unable to create file");

    // Write the header line
    writeln!(file, "mutation").expect("Unable to write to file");

    for s in sums.iter() {
        writeln!(file, "{}", s).expect("Unable to write to file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_readfile() {
        let filename1 = "./test_resources/dat1.json";
        let filename2 = "./test_resources/dat2.json";
        let varname = "transport";

        // Test when nentries <= 0
        let nentries = 0;
        let result = std::panic::catch_unwind(|| {
            readfile(filename1, varname, nentries);
        });
        assert!(result.is_err(), "Expected an error when nentries <= 0");

        let result = std::panic::catch_unwind(|| {
            readfile(filename1, "nonexistent_var", nentries);
        });
        assert!(
            result.is_err(),
            "Expected an error when varname does not exist"
        );

        let result = std::panic::catch_unwind(|| {
            readfile(filename1, varname, 0);
        });
        assert!(result.is_err(), "Expected an error when nentries <= 0");

        let nentries = 10;

        let (index1, values1, _groups1) = readfile(filename1, varname, nentries);
        let (index2, values2, _groups2) = readfile(filename2, varname, nentries);

        assert_eq!(
            index1.len(),
            nentries,
            "The lengths of index1 and values1 are not equal"
        );
        assert_eq!(
            values1.len(),
            nentries,
            "The lengths of index1 and values1 are not equal"
        );
        assert_eq!(
            index2.len(),
            nentries,
            "The lengths of index2 and values2 are not equal"
        );
        assert_eq!(
            values2.len(),
            nentries,
            "The lengths of values1 and values2 are not equal"
        );
        assert!(
            values1.iter().tuple_windows().all(|(a, b)| a <= b),
            "values1 is not sorted"
        );
        assert!(
            values2.iter().tuple_windows().all(|(a, b)| a <= b),
            "values2 is not sorted"
        );
    }

    #[test]
    fn test_write_file() {
        use std::fs;
        use std::io::Read;

        let sums = vec![1.0, 4.5, 3.0, 2.0];
        let filename = "/tmp/test_write_file.txt";

        write_file(&sums, filename);

        let mut file = fs::File::open(filename).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let expected_contents = "\
            mutation\n\
            1\n\
            4.5\n\
            3\n\
            2\n";

        assert_eq!(contents, expected_contents);
    }
}
