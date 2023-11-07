use itertools::Itertools;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

pub fn readfile(filename: &str, varname: &str, nentries: usize) -> (Vec<usize>, Vec<f64>) {
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
                if let Some(value) = map.get(varname) {
                    if let Value::Number(number) = value {
                        if let Some(number) = number.as_f64() {
                            values.push(number);
                        }
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

pub fn write_file(
    values1: &Vec<f64>,
    values2: &Vec<f64>,
    diffs_abs: &Vec<f64>,
    diffs_rel: &Vec<f64>,
    index1: &Vec<usize>,
    index2: &Vec<usize>,
    ord_index: &Vec<usize>,
    filename: &str,
) {
    const ERR_MSG: &str = "All input vectors must have the same length";
    let len = values1.len();
    assert_eq!(values2.len(), len, "{}", ERR_MSG);
    assert_eq!(diffs_abs.len(), len, "{}", ERR_MSG);
    assert_eq!(diffs_rel.len(), len, "{}", ERR_MSG);
    assert_eq!(index1.len(), len, "{}", ERR_MSG);
    assert_eq!(index2.len(), len, "{}", ERR_MSG);
    assert_eq!(ord_index.len(), len, "{}", ERR_MSG);

    let mut file = File::create(filename).expect("Unable to create file");

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

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_readfile() {
        let filename1 = "./test_resources/dat1.json";
        let filename2 = "./test_resources/dat2.json";
        let varname = "transport";
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
}
