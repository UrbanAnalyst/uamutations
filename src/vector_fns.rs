use ndarray::{s, Array2};

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
/// # Returns
///
/// A vector of f64 values representing the sequential differences between `values1` and `values2`.
/// If `absolute` is true, the differences are absolute values. Otherwise, the differences are
/// differences relative to `values1`.
///
/// # Example
///
/// ```
/// use uamutations::vector_fns::calculate_dists;
/// let values1 = ndarray::array![[1.0, 2.0, 4.0, 5.0]];
/// let values2 = ndarray::array![[2.0, 3.0, 7.0, 9.0]];
/// let result = calculate_dists(&values1, &values2, true);
/// // For each values1, result will be (v2 - v1) for closest values2. So closest value to v1[3] =
/// // 4, for example, is v2 = 3, and (v2 - v1) = 3 - 4 = -1. Or v1[4] = 5, with closest of 3, and
/// // 3 - 5 = -2.
/// assert_eq!(result, vec![1.0, 0.0, -1.0, -2.0]);
/// let result = calculate_dists(&values1, &values2, false);
/// assert_eq!(result, vec![1.0, 0.0, -0.25, -0.4]);
/// ```
pub fn calculate_dists(values1: &Array2<f64>, values2: &Array2<f64>, absolute: bool) -> Vec<f64> {
    assert!(!values1.is_empty(), "values1 must not be empty");
    assert_eq!(
        values1.dim(),
        values2.dim(),
        "values1 and values2 must have the same dimensions."
    );

    let values1_clone = values1.t().to_owned();
    let values2_clone = values2.t().to_owned();

    // Make a vector of (distances, index) from each `values1` entry to the closest entry of
    // `values2` in the multi-dimensional space defined by each array.
    let mut results: Vec<usize> = Vec::new();

    for v1 in values1_clone.outer_iter() {
        let mut min_dist = f64::MAX;
        let mut min_index = 0;

        for (j, v2) in values2_clone.outer_iter().enumerate() {
            let dist = v1
                .iter()
                .zip(v2.iter())
                .map(|(&a, &b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();

            if dist < min_dist {
                min_dist = dist;
                min_index = j;
            }
        }

        results.push(min_index);
    }

    // Then calculate final distances from each item in the first dimension of `values1` to the
    // first dimension of `values2` of the item which is closest in the full multi-dimensional
    // space.
    let mut final_results: Vec<f64> = Vec::new();

    for (&min_index, v1) in results.iter().zip(values1_clone.outer_iter()) {
        let v2 = values2_clone.slice(s![min_index, ..]);
        let dist = if absolute {
            v2[0] - v1[0]
        } else {
            (v2[0] - v1[0]) / v1[0]
        };
        final_results.push(dist);
    }

    final_results
}

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
/// use uamutations::vector_fns::get_ordering_index;
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
    fn test_calculate_dists_absolute() {
        // let _values1 = vec![vec![1.0, 2.0, 4.0, 5.0]];
        // let _values2 = vec![vec![2.0, 3.0, 7.0, 9.0]];
        // For each values1, result will be (v2 - v1) for closest values2. So closest value to
        // v1[3] = 4, for example, is v2 = 3, and (v2 - v1) = 3 - 4 = -1. Or v1[4] = 5, with
        // closest of 3, and 3 - 5 = -2.
        // let _expected = vec![1.0, 0.0, -1.0, -2.0]; // values2 - values1
        // assert_eq!(calculate_dists(&values1, &values2, true), expected);
    }

    #[test]
    fn test_calculate_dists_relative() {
        // let _values1 = vec![vec![1.0, 2.0, 4.0, 5.0]];
        // let _values2 = vec![vec![2.0, 3.0, 7.0, 9.0]];
        // Closest values are same as above, but calculated here as relative values, so -1 becomes
        // -1/4, and -2 becomes -2/5.
        // let _expected = vec![1.0, 0.0, -0.25, -0.4]; // (values2 - values1) / values1
        // assert_eq!(calculate_dists(&values1, &values2, false), expected);
    }
}
