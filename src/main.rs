mod readfile;

const NENTRIES: usize = 100;
const FNAME1: &str = "dat1.json";
const FNAME2: &str = "dat2.json";
const VARNAME: &str = "transport";

fn main() {
    let mut values1 = readfile::readfile(FNAME1, VARNAME, NENTRIES);
    let mut values2 = readfile::readfile(FNAME2, VARNAME, NENTRIES);
    values1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    values2.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for ((index1, number1), (_, number2)) in values1.iter().enumerate().zip(values2.iter().enumerate()) {
        println!("Index: {}: ( {}, {} )", index1 + 1, number1, number2);
    }
}
