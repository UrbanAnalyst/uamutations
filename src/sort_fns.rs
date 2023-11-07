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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ordering_index() {
        let vals = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let expected = vec![4, 2, 0, 1, 3]; // Indices of vals sorted in descending order
        assert_eq!(get_ordering_index(&vals, false), expected);
    }

    #[test]
    fn test_get_ordering_index_abs() {
        let vals = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let expected = vec![4, 3, 2, 1, 0]; // Indices of absolute values of vals sorted in descending order
        assert_eq!(get_ordering_index(&vals, true), expected);
    }
}
