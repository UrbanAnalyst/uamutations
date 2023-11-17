use ndarray::{s, Array2};
use ndarray_linalg::LeastSquaresSvd;

/// Calculates beta coefficients (slopes) of a multiple linear regression of dimensions [1.., _] of
/// input array against first dimension [0, _].
///
/// # Arguments
///
/// * `data` - An ndarray::Array2 object of [variables, observations].
///
/// # Panics
///
/// This function will panic if `data` is empty.
///
/// # Returns
///
/// Vector of f64 values of multiple linear regression coefficients, one for each variable.
///
/// # Example
///
/// ```
/// // use uamutations::vector_fns::calculate_dists;
/// let values1 = vec![vec![1.0, 2.0, 4.0, 5.0]];
/// let values2 = vec![vec![2.0, 3.0, 7.0, 9.0]];
/// // let result = calculate_dists(&values1, &values2, true);
/// // For each values1, result will be (v2 - v1) for closest values2. So closest value to v1[3] =
/// // 4, for example, is v2 = 3, and (v2 - v1) = 3 - 4 = -1. Or v1[4] = 5, with closest of 3, and
/// // 3 - 5 = -2.
/// // assert_eq!(result, vec![1.0, 0.0, -1.0, -2.0]);
/// // let result = calculate_dists(&values1, &values2, false);
/// // assert_eq!(result, vec![1.0, 0.0, -0.25, -0.4]);
/// ```
pub fn mlr_beta(data: &Array2<f64>) -> Vec<f64> {
    assert!(!data.is_empty(), "values1 must not be empty");

    let data_t = data.t();
    let target_var = data_t.column(0).to_owned();
    let data_t = data_t.slice(s![.., 1..]).to_owned();

    let result = data_t.least_squares(&target_var).unwrap();
    let b = result.solution;
    println!("{:?}", b);
    // println!("b has {:?} dimensions", b.dim());

    b.to_vec()
}
