pub fn get_ordering_index(vals: &Vec<f64>) -> Vec<usize> {

    let mut pairs: Vec<_> = vals.clone().into_iter().enumerate().collect();
    pairs.sort_by(|&(_, a), &(_, b)| b.abs().partial_cmp(&a.abs()).unwrap());

    let index: Vec<_> = pairs.iter().map(|&(index, _)| index).collect();
    index
}

