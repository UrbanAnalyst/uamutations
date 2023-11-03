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
