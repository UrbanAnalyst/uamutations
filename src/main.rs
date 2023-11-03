mod readfile;

const NENTRIES: usize = 100;
const FNAME1: &str = "dat1.json";
const FNAME2: &str = "dat2.json";
const VARNAME: &str = "transport";

fn main() {
    let values1 = readfile::readfile(FNAME1, VARNAME, NENTRIES);
    let values2 = readfile::readfile(FNAME2, VARNAME, NENTRIES);

    for ((index1, number1), (_, number2)) in values1.iter().enumerate().zip(values2.iter().enumerate()) {
        println!("Index: {}: ( {}, {} )", index1 + 1, number1, number2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_values_sorted() {
        let values1 = readfile::readfile(FNAME1, VARNAME, NENTRIES);
        let values2 = readfile::readfile(FNAME2, VARNAME, NENTRIES);

        assert!(values1.iter().tuple_windows().all(|(a, b)| a <= b), "values1 is not sorted");
        assert!(values2.iter().tuple_windows().all(|(a, b)| a <= b), "values2 is not sorted");
    }
}