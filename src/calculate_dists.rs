use ndarray::Array2;

pub struct OrderingIndex {
    index_sort: Vec<usize>,
    index_reorder: Vec<usize>,
}

/// Calculates a vector of sequential difference between two arrays of f64 values.
///
/// The distances are calculated in the full multi-dimensional space, so that each value in the
/// first array (`values1`) is matched to the entry in `values2` at the minimal distance. Each
/// element of `values2` is matched to one unique element of `values1`. Unique matching is done
/// with a hash map, meaning that the procedure only works consistently if started from some
/// extreme point of the `values1` distribution. This extreme point is taken as the lowest value of
/// the first column of `values1`. Alternatives to this include calculating a multiple linear
/// regression between the first column of `values1` and all columns of `values2`, and taking the
/// first point of that, but that will by definition be the lowest (or possibly highest) value of
/// `values1` anyway.
///
/// This consideration means that `values1` can first be sorted by the first column, these sorted
/// values used to find closest values of `values2`, and then the original order restored to yield
/// the final desired matching.
///
/// # Arguments
///
/// * `values1` - An Array2 object which provides the reference values against which to sort
/// `values2`.
/// * `values2` - An Array2 object which is to be sorted against `values1`.
/// * `absolute` - A boolean indicating whether to calculate absolute differences.
///
/// # Panics
///
/// This function will panic if `values1` is empty or if `values1` and `values2` have different
/// dimensions.
///
/// # Returns
///
/// A vector of `usize` values matching each consecutive element in `values1` to the closest
/// elements in `values2`.  If `absolute` is true, the differences are absolute values. Otherwise,
/// the differences are differences relative to `values1`.
///
/// # Example
///
/// ```
/// use uamutations::calculate_dists::calculate_dists;
/// let values1 = ndarray::array![[1.0, 2.0, 4.0, 5.0]];
/// let values2 = ndarray::array![[7.0, 9.0, 3.0, 2.0]];
/// let result = calculate_dists(&values1, &values2, true);
/// // For each values1, result will be (v2 - v1) for closest values2. So closest value to v1[3] =
/// // 4, for example, is v2 = 3, and (v2 - v1) = 3 - 4 = -1. Or v1[4] = 5, with closest of 3, and
/// // 3 - 5 = -2.
/// assert_eq!(result, vec![1.0, 1.0, 3.0, 4.0]);
/// let result = calculate_dists(&values1, &values2, false);
/// assert_eq!(result, vec![1.0, 0.5, 0.75, 0.8]);
/// ```
pub fn calculate_dists(values1: &Array2<f64>, values2: &Array2<f64>, absolute: bool) -> Vec<f64> {
    assert!(!values1.is_empty(), "values1 must not be empty");
    assert_eq!(
        values1.dim(),
        values2.dim(),
        "values1 and values2 must have the same dimensions."
    );

    let values1_ref_var: Vec<f64> = values1.row(0).to_vec();
    let values2_ref_var: Vec<f64> = values2.row(0).to_vec();

    let sorting_order = get_ordering_index(&values1_ref_var.to_vec(), false, false);

    // Order values1_ref_var by sorting_order.index_sort:
    let values1_sorted: Vec<f64> = sorting_order
        .index_sort
        .iter()
        .map(|&i| values1_ref_var[i])
        .collect();
    // Sort values2_ref_var:
    let mut values2_sorted = values2_ref_var.clone();
    values2_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Re-order values2_ref_var for minimal overal diff to values1:
    let values2_sorted: Vec<f64> = reorder_min_diff(&values1_sorted, &values2_sorted);

    // Calculate conseqcutive differences between the two vectors:
    let differences: Vec<f64> = values1_sorted
        .iter()
        .zip(values2_sorted.iter())
        .map(|(&a, &b)| if absolute { b - a } else { (b - a) / a })
        .collect();
    // And re-order those differences according to sorting_order.index_reorder, so they align with
    // the original order of `values1`:
    let differences: Vec<f64> = sorting_order
        .index_reorder
        .iter()
        .map(|&i| differences[i])
        .collect();

    differences
}

/// Returns a vector of indices that would sort the input vector in ascending or descending order.
///
/// # Arguments
///
/// * `vals` - A slice of f64 values to be sorted.
/// * `desc` - If `true`, sort in descending order; otherwise sort in ascensing order.
/// * `is_abs` - A boolean indicating whether sorting should be based on absolute values.
///
/// # Example
///
/// ```
/// use uamutations::calculate_dists::get_ordering_index;
/// let vals = vec![1.0, -2.0, 3.0, -4.0, 5.0];
/// let result = get_ordering_index(&vals, false, false);
/// // assert_eq!(result, vec![3, 1, 0, 2, 4]);
/// ```
pub fn get_ordering_index(vals: &[f64], desc: bool, is_abs: bool) -> OrderingIndex {
    let mut pairs: Vec<_> = vals.iter().enumerate().collect();

    if is_abs {
        if desc {
            pairs.sort_by(|&(_, a), &(_, b)| b.abs().partial_cmp(&a.abs()).unwrap());
        } else {
            pairs.sort_by(|&(_, a), &(_, b)| a.abs().partial_cmp(&b.abs()).unwrap());
        }
    } else if desc {
        pairs.sort_by(|&(_, a), &(_, b)| b.partial_cmp(a).unwrap());
    } else {
        pairs.sort_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap());
    }

    let index: Vec<_> = pairs.iter().map(|&(index, _)| index).collect();

    let mut reorder_index = vec![0; index.len()];
    for (i, &idx) in index.iter().enumerate() {
        reorder_index[idx] = i;
    }

    OrderingIndex {
        index_sort: index,
        index_reorder: reorder_index,
    }
}

/// Order one input vector against another to acheive the minimal overal difference between them.
///
/// # Arguments
///
/// * `arr1` - A *sorted* array of f64 values
/// * `arr2` - A *sorted* array of f64 values.
///
/// # Returns
///
/// * A sorted version of `arr2` that achieves the minimal overall difference with `arr1`.
fn reorder_min_diff(arr1: &[f64], arr2: &[f64]) -> Vec<f64> {
    let n = arr1.len();
    let mut dp = vec![vec![0.0; n + 1]; n + 1];
    let mut pairs = vec![vec![(0.0, 0.0); n + 1]; n + 1];

    // Create a vector of tuples (value, original position)
    let arr1_with_pos: Vec<(f64, usize)> = arr1
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    for i in 1..=n {
        for j in 1..=n {
            if dp[i - 1][j - 1] + (arr1_with_pos[i - 1].0 - arr2[j - 1]).abs()
                < f64::min(dp[i - 1][j], dp[i][j - 1])
            {
                dp[i][j] = dp[i - 1][j - 1] + (arr1_with_pos[i - 1].0 - arr2[j - 1]).abs();
                pairs[i][j] = (arr1_with_pos[i - 1].0, arr2[j - 1]);
            } else if dp[i - 1][j] < dp[i][j - 1] {
                dp[i][j] = dp[i - 1][j];
                pairs[i][j] = pairs[i - 1][j];
            } else {
                dp[i][j] = dp[i][j - 1];
                pairs[i][j] = pairs[i][j - 1];
            }
        }
    }

    let mut ordered_arr2 = vec![0.0; n];
    let mut i = n;
    let mut j = n;
    while i > 0 && j > 0 {
        if pairs[i][j] != pairs[i - 1][j] {
            // Use the recorded position to reorder ordered_arr2
            ordered_arr2[arr1_with_pos[i - 1].1] = pairs[i][j].1;
            j -= 1;
        }
        i -= 1;
    }

    ordered_arr2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ordering_index() {
        let vals = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let expected = OrderingIndex {
            index_sort: vec![3, 1, 0, 2, 4], // Indices of vals in ascending order
            // bool flags are (desc, is_abs):
            index_reorder: vec![2, 1, 3, 0, 4],
        };
        let oi = get_ordering_index(&vals, false, false);
        assert_eq!(oi.index_sort, expected.index_sort);
        assert_eq!(oi.index_reorder, expected.index_reorder);
    }

    #[test]
    fn test_get_ordering_index_abs() {
        let vals = vec![1.0, -2.0, 3.0, -4.0, 5.0];
        let expected = OrderingIndex {
            index_sort: vec![0, 1, 2, 3, 4], // Indices of vals in ascending order
            // bool flags are (desc, is_abs):
            index_reorder: vec![0, 1, 2, 3, 4],
        };
        let oi = get_ordering_index(&vals, false, true);
        assert_eq!(oi.index_sort, expected.index_sort);
        assert_eq!(oi.index_reorder, expected.index_reorder);
    }

    #[test]
    fn test_calculate_dists_absolute() {
        // Note that 2.0 is closest to 2.0, but is matched to 3.0 because of sequential and unique
        // matching.
        let values1 = ndarray::array![[1.0, 2.0, 4.0, 5.0]];
        let values2 = ndarray::array![[7.0, 9.0, 3.0, 2.0]];
        let result = calculate_dists(&values1, &values2, true);
        assert_eq!(result, vec![1.0, 1.0, 3.0, 4.0]);
    }

    #[test]
    fn test_calculate_dists_relative() {
        let values1 = ndarray::array![[1.0, 2.0, 4.0, 5.0]];
        let values2 = ndarray::array![[7.0, 9.0, 3.0, 2.0]];
        let result = calculate_dists(&values1, &values2, false);
        assert_eq!(result, vec![1.0, 0.5, 0.75, 0.8]);
    }
}
