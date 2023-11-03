mod readfile;

const NENTRIES: usize = 100;
const FILENAME: &str = "dat1.json";
const VARNAME: &str = "transport";

fn main() {
    let values = readfile::readfile(FILENAME, VARNAME, NENTRIES);

    for (index, number) in values.iter().enumerate() {
        println!("{}: {}", index + 1, number);
    }
}
