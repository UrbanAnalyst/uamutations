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
/// use ndarray::array;
/// use uamutations::mlr::mlr_beta;
/// // Example with 2 variables
/// let data_2 = array![
/// [1.0, 2.0, 3.0, 4.0, 5.0],
/// [2.1, 3.2, 4.1, 5.2, 5.9],
/// ];
/// let result_2 = mlr_beta(&data_2);
/// println!("Result with 2 variables: {:?}", result_2);
///
/// // Example with 3 variables
/// let data_3 = array![
/// [1.0, 2.0, 3.0, 4.0, 5.0],
/// [2.1, 3.2, 4.1, 5.2, 5.9],
/// [3.0, 4.1, 4.9, 6.0, 7.1],
/// ];
/// let result_3 = mlr_beta(&data_3);
/// println!("Result with 3 variables: {:?}", result_3);
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

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_mlr_beta_2_variables() {
        let data_2 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [2.1, 3.2, 4.1, 5.2, 5.9],];
        let result_2 = mlr_beta(&data_2);
        assert_eq!(result_2.len(), 1);
    }

    #[test]
    fn test_mlr_beta_3_variables() {
        let data_3 = array![
            [1.0, 2.0, 3.0, 4.0, 5.0],
            [2.1, 3.2, 4.1, 5.2, 5.9],
            [3.0, 4.1, 4.9, 6.0, 7.1],
        ];
        let result_3 = mlr_beta(&data_3);
        assert_eq!(result_3.len(), 2);
    }
}
