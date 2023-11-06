use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

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

    let mut pairs: Vec<_> = values.clone().into_iter().enumerate().collect();
    pairs.sort_by(|&(_, a), &(_, b)| a.partial_cmp(&b).unwrap());

    let indices: Vec<usize> = pairs.iter().map(|&(index, _)| index).collect();

    (indices, values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readfile() {
        let filename = "dat1.json";
        let varname = "transport";
        let nentries = 10;

        let (indices, result) = readfile(filename, varname, nentries);

        assert_eq!(result.len(), nentries);
        assert_eq!(indices.len(), nentries);

        for value in &result {
            assert!(*value >= 0.0, "Found value less than 0");
        }
    }
}
