pub fn get_ordering_index(vals: &Vec<f64>, is_abs: bool) -> Vec<usize> {
    let mut pairs: Vec<_> = vals.clone().into_iter().enumerate().collect();

    if is_abs {
        pairs.sort_by(|&(_, a), &(_, b)| b.abs().partial_cmp(&a.abs()).unwrap());
    } else {
        pairs.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
    }

    let index: Vec<_> = pairs.iter().map(|&(index, _)| index).collect();
    index
}
