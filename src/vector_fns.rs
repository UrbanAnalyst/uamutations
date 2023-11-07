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
        let expected = vec![4, 2, 0, 1, 3]; // Indices of vals in descending order
        assert_eq!(get_ordering_index(&vals, false), expected);
    }

    #[test]
    fn test_get_ordering_index_abs() {
        let vals = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let expected = vec![4, 3, 2, 1, 0]; // Indices of absolute vals in descending order
        assert_eq!(get_ordering_index(&vals, true), expected);
    }
}

pub fn calculate_diffs(values1: &Vec<f64>, values2: &Vec<f64>, absolute: bool) -> Vec<f64> {
    // Full calls for the two cases, because `if`/`else` clauses require same type, and the `map`
    // calls generate different types.
    if absolute {
        values1
            .iter()
            .zip(values2.iter())
            .map(|(&x, &y)| y - x)
            .collect()
    } else {
        values1
            .iter()
            .zip(values2.iter())
            .map(|(&x, &y)| (y - x) / (x + y))
            .collect()
    }
}
