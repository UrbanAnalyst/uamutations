use std::fs::File;
use std::io::Write;

mod readfile;
mod sort_fns;

const NENTRIES: usize = 1000;
const FNAME1: &str = "dat1.json";
const FNAME2: &str = "dat2.json";
const VARNAME: &str = "bike_index";

fn main() {

    let (index1, values1) = readfile::readfile(FNAME1, VARNAME, NENTRIES);
    let (index2, values2) = readfile::readfile(FNAME2, VARNAME, NENTRIES);

    assert_eq!(index1.len(), values1.len(), "The lengths of index1 and values1 are not equal");
    assert_eq!(index2.len(), values2.len(), "The lengths of index2 and values2 are not equal");
    assert_eq!(values1.len(), values2.len(), "The lengths of values1 and values2 are not equal");
    
    let diffs_abs: Vec<_> = values1.iter()
        .zip(values2.iter())
        .map(|(&x, &y)| y - x)
        .collect();

    let diffs_rel: Vec<_> = values1.iter()
        .zip(values2.iter())
        .map(|(&x, &y)| (y - x) / (x + y))
        .collect();

    assert_eq!(values1.len(), diffs_abs.len(), "The lengths of values1 and differences are not equal");
    assert_eq!(values1.len(), diffs_rel.len(), "The lengths of values1 and differences are not equal");

    let ord_index = sort_fns::get_ordering_index(&diffs_rel, true); // true for is_abs

    let mut file = File::create("output.txt").expect("Unable to create file");

for ((((((number1, number2), dabs), drel), i1), i2), oi) in 
    values1.iter()
        .zip(values2.iter())
        .zip(diffs_abs.iter())
        .zip(diffs_rel.iter())
        .zip(index1.iter())
        .zip(index2.iter())
        .zip(ord_index.iter()) 
    {
        write!(file, "{}, {}, {}, {}, {}, {}, {}\n", number1, number2, dabs, drel, i1, i2, oi)
            .expect("Unable to write to file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_values_sorted() {
        let (index1, values1) = readfile::readfile(FNAME1, VARNAME, NENTRIES);
        let (index2, values2) = readfile::readfile(FNAME2, VARNAME, NENTRIES);

        assert_eq!(index1.len(), values1.len(), "The lengths of index1 and values1 are not equal");
        assert_eq!(index2.len(), values2.len(), "The lengths of index2 and values2 are not equal");
        assert_eq!(values1.len(), values2.len(), "The lengths of values1 and values2 are not equal");
        assert!(values1.iter().tuple_windows().all(|(a, b)| a <= b), "values1 is not sorted");
        assert!(values2.iter().tuple_windows().all(|(a, b)| a <= b), "values2 is not sorted");
    }
}
