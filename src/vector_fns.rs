/// Returns a vector of indices that would sort the input vector in descending order.
///
/// # Arguments
///
/// * `vals` - A slice of f64 values to be sorted.
/// * `is_abs` - A boolean indicating whether sorting should be based on absolute values.
///
/// # Example
///
/// ```
/// let vals = vec![1.0, -2.0, 3.0, -4.0, 5.0];
/// let result = get_ordering_index(&vals, false);
/// assert_eq!(result, vec![4, 2, 0, 1, 3]);
/// ```
pub fn get_ordering_index(vals: &[f64], is_abs: bool) -> Vec<usize> {
    let mut pairs: Vec<_> = vals.iter().enumerate().collect();

    if is_abs {
        pairs.sort_by(|&(_, a), &(_, b)| b.abs().partial_cmp(&a.abs()).unwrap());
    } else {
        pairs.sort_by(|&(_, a), &(_, b)| b.partial_cmp(a).unwrap());
    }

    let index: Vec<_> = pairs.iter().map(|&(index, _)| index).collect();

    index
}

/// Calculates a vector of sequential difference between two vectors of f64 values.
///
/// # Arguments
///
/// * `values1` - The first vector of f64 values.
/// * `values2` - The second vector of f64 values.
/// * `absolute` - A boolean indicating whether to calculate absolute differences.
///
/// # Panics
///
/// This function will panic if `values1` is empty or if `values1` and `values2` have different
/// lengths.
///
/// # Example
///
/// ```
/// let values1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let values2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
/// let result = calculate_diffs(&values1, &values2, true);
/// assert_eq!(result, vec![1.0, 1.0, 1.0, 1.0, 1.0]);
/// ```
pub fn calculate_diffs(values1: &Vec<f64>, values2: &Vec<f64>, absolute: bool) -> Vec<f64> {
    assert!(!values1.is_empty(), "values1 must not be empty");
    assert_eq!(
        values1.len(),
        values2.len(),
        "values1 and values2 must have the same length"
    );

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

    #[test]
    fn test_calculate_diffs_absolute() {
        let values1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let values2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let expected = vec![1.0, 1.0, 1.0, 1.0, 1.0]; // Differences between values2 and values1
        assert_eq!(calculate_diffs(&values1, &values2, true), expected);
    }

    #[test]
    fn test_calculate_diffs_relative() {
        let values1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let values2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let expected = vec![
            0.3333333333333333,
            0.2,
            0.14285714285714285,
            0.1111111111111111,
            0.09090909090909091,
        ]; // Relative differences between values2 and values1
        assert_eq!(calculate_diffs(&values1, &values2, false), expected);
    }
}
