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

    if let Value::Array(array) = &json {
        for item in array {
            if values.len() >= nentries {
                break;
            }
            if let Value::Object(map) = item {
                if let Some(Value::Number(number)) = map.get(varname) {
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

/// `WriteData` is a struct that holds the data needed for the [`write_file`] function which writes
/// data to a local file.
///
/// It contains two sets of values (`values1` and `values2`), their absolute and relative
/// differences (`diffs_abs` and `diffs_rel`), and their original indices (`index1` and `index2`).
///
/// # Fields
///
/// * `values` - Vector of values which are to be mutated, in original order.
/// * `diffs` - Correponding vector of relative differences following mutation.
///
/// [`write_file`]: fn.write_file.html
pub struct WriteData {
    pub values: Vec<f64>,
    pub diffs: Vec<f64>,
    pub groups: Vec<usize>,
}

/// Writes the data contained in a [`WriteData`] instance plus one additional vector to a file.
///
/// The function takes a reference to a [`WriteData`] instance, a reference to a vector of ordering
/// indices, and a filename as arguments.
///
/// # Arguments
///
/// * `data` - A reference to a [`WriteData`] instance containing the data to be written.
/// * `filename` - The name of the file to which the data will be written.
///
/// # Panics
///
/// This function will panic if it fails to create or write to the file.
///
/// [`WriteData`]: struct.WriteData.html
pub fn write_file(data: &WriteData, filename: &str) {
    const ERR_MSG: &str = "All input vectors must have the same length";
    assert_eq!(data.values.len(), data.diffs.len(), "{}", ERR_MSG);

    let mut file = File::create(filename).expect("Unable to create file");

    // Write the header line
    writeln!(file, "values, diffs, groups").expect("Unable to write to file");

    for ((v, d), g) in data.values.iter().zip(data.diffs.iter()).zip(data.groups.iter()) {
        writeln!(file, "{}, {}, {}", v, d, g).expect("Unable to write to file");
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

        let testdata = WriteData {
            values: vec![1.0, 4.5, 3.0, 2.0],
            diffs: vec![4.0, 12.0, 6.0, 5.0],
            groups: vec![1, 1, 2, 2],
        };
        let filename = "/tmp/test_write_file.txt";

        write_file(&testdata, filename);

        let mut file = fs::File::open(filename).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let expected_contents = "\
            values, diffs, groups\n\
            1, 4, 1\n\
            4.5, 12, 1\n\
            3, 6, 2\n\
            2, 5, 2\n";

        assert_eq!(contents, expected_contents);
    }
}
