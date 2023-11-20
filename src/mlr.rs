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
    println!("------------");
    println!("data has {:?} dimensions", data.dim());

    // Transpose data(vars, obs) to (obs, vars):
    let mut data_clone = data.t().to_owned();
    println!("data_clone starts with {:?} dimensions", data_clone.dim());
    // Take first column as target_var:
    let target_var = data_clone.column(0).to_owned();
    println!("target_var has {:?} dimensions", target_var.dim());
    // Remove that column from data_clone:
    // data_clone.slice_mut(s![.., 1..]);
    data_clone = data_clone.slice(s![.., 1..]).to_owned();
    // let _dsq = data.t().dot(data);
    println!("data_clone has {:?} dimensions", data_clone.dim());

    // The least squares regression call:
    let result = data_clone.least_squares(&target_var).unwrap();
    let b: Vec<f64> = result.solution.to_vec();
    println!("{:?}", b);
    println!("b has {:?} length", b.len());
    println!("------------");
    println!();

    b
}
