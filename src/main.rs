mod read_write_file;
mod vector_fns;

const NENTRIES: usize = 1000;
const FNAME1: &str = "dat1.json";
const FNAME2: &str = "dat2.json";
const VARNAME: &str = "bike_index";
const OUTFILENAME: &str = "output.txt";

fn main() {
    let (index1, values1) = read_write_file::readfile(FNAME1, VARNAME, NENTRIES);
    let (index2, values2) = read_write_file::readfile(FNAME2, VARNAME, NENTRIES);

    let diffs_abs = vector_fns::calculate_diffs(&values1, &values2, true);
    let diffs_rel = vector_fns::calculate_diffs(&values1, &values2, false);

    let ord_index = vector_fns::get_ordering_index(&diffs_rel, true); // true for is_abs

    read_write_file::write_file(
        &values1,
        &values2,
        &diffs_abs,
        &diffs_rel,
        &index1,
        &index2,
        &ord_index,
        OUTFILENAME,
    );
}
