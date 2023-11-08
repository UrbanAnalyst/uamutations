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
/// let filename = "./test_resources/dat1.json";
/// let varname = "transport";
/// let nentries = 10;
/// let (index, values) = readfile(filename, varname, nentries);
/// ```

pub fn readfile(filename: &str, varname: &str, nentries: usize) -> (Vec<usize>, Vec<f64>) {
    assert!(nentries > 0, "nentries must be greater than zero");

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader).unwrap();

    let mut values = Vec::new();

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
            }
        }
    }

    let mut values: Vec<_> = values.into_iter().enumerate().collect();
    values.sort_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());

    let index: Vec<usize> = values.iter().map(|&(index, _)| index).collect();
    let values: Vec<f64> = values.iter().map(|&(_, value)| value).collect();

    assert_eq!(
        index.len(),
        values.len(),
        "The lengths of index and values are not equal"
    );
    assert!(
        values.iter().tuple_windows().all(|(a, b)| a <= b),
        "values are not sorted"
    );

    (index, values)
}

/// `WriteData` is a struct that holds the data to be written to a file.
///
/// It contains two sets of values (`values1` and `values2`), their absolute and relative
/// differences (`diffs_abs` and `diffs_rel`), and their original indices (`index1` and `index2`).
///
/// # Fields
///
/// * `values1` - Ordered vector of values which are to be changed = "from" values
/// * `index1` - Index mapping back to original order of 'values1'
/// * `values2` - Ordered vector of values to be mutated towards = "to" values
/// * `index2` - Index mapping back to original order of 'values2'
/// * `diffs_abs` - Vector of absolute differences between 'values1' and 'values2'
/// * `diffs_rel` - Vector of relative differences between 'values1' and 'values2'
pub struct WriteData {
    pub values1: Vec<f64>,
    pub values2: Vec<f64>,
    pub diffs_abs: Vec<f64>,
    pub diffs_rel: Vec<f64>,
    pub index1: Vec<usize>,
    pub index2: Vec<usize>,
}

/// Writes the data contained in a `WriteData` instance to a file.
///
/// The function takes a reference to a `WriteData` instance, a reference to a vector of ordering
/// indices, and a filename as arguments. It writes the data to the file in the following format:
/// `values1`, `values2`, `diffs_abs`, `diffs_rel`, `index1`, `index2`, `ord_index`.
///
/// # Arguments
///
/// * `data` - A reference to a `WriteData` instance containing the data described in the struct.
/// * `ord_index` - A reference to a vector of ordering indices mapping order of 'diffs_rel' back
/// onto original order of 'values1'.
/// * `filename` - The name of the file to which the data will be written.
///
/// # Panics
///
/// This function will panic if it fails to create or write to the file.
pub fn write_file(data: &WriteData, ord_index: &Vec<usize>, filename: &str) {
    const ERR_MSG: &str = "All input vectors must have the same length";
    let len = data.values1.len();
    assert_eq!(data.values2.len(), len, "{}", ERR_MSG);
    assert_eq!(data.diffs_abs.len(), len, "{}", ERR_MSG);
    assert_eq!(data.diffs_rel.len(), len, "{}", ERR_MSG);
    assert_eq!(data.index1.len(), len, "{}", ERR_MSG);
    assert_eq!(data.index2.len(), len, "{}", ERR_MSG);
    assert_eq!(ord_index.len(), len, "{}", ERR_MSG);

    let mut file = File::create(filename).expect("Unable to create file");

    for ((((((number1, number2), dabs), drel), i1), i2), oi) in data
        .values1
        .iter()
        .zip(data.values2.iter())
        .zip(data.diffs_abs.iter())
        .zip(data.diffs_rel.iter())
        .zip(data.index1.iter())
        .zip(data.index2.iter())
        .zip(ord_index.iter())
    {
        writeln!(
            file,
            "{}, {}, {}, {}, {}, {}, {}",
            number1, number2, dabs, drel, i1, i2, oi
        )
        .expect("Unable to write to file");
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

        let (index1, values1) = readfile(filename1, varname, nentries);
        let (index2, values2) = readfile(filename2, varname, nentries);

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

        for value in &values1 {
            assert!(*value >= 0.0, "Found value less than 0");
        }
        for value in &values2 {
            assert!(*value >= 0.0, "Found value less than 0");
        }
    }

    #[test]
    fn test_write_file() {
        use std::fs;
        use std::io::Read;

        let testdata = WriteData {
            values1: vec![1.0, 2.0, 3.0],
            values2: vec![4.0, 5.0, 6.0],
            diffs_abs: vec![7.0, 8.0, 9.0],
            diffs_rel: vec![10.0, 11.0, 12.0],
            index1: vec![13, 14, 15],
            index2: vec![16, 17, 18],
        };
        let ord_index = vec![19, 20, 21];
        let filename = "/tmp/test_write_file.txt";

        write_file(&testdata, &ord_index, filename);

        let mut file = fs::File::open(filename).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let expected_contents = "\
            1, 4, 7, 10, 13, 16, 19\n\
            2, 5, 8, 11, 14, 17, 20\n\
            3, 6, 9, 12, 15, 18, 21\n";

        assert_eq!(contents, expected_contents);
    }
}
