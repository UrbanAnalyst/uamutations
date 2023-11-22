use ndarray::{s, Array2, Axis};
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

    // Transpose data(vars, obs) to (obs, vars):
    let mut data_clone = data.t().to_owned();
    // Take first column as target_var:
    let target_var = data_clone.column(0).to_owned();
    // Remove that column from data_clone:
    // data_clone.slice_mut(s![.., 1..]);
    data_clone = data_clone.slice(s![.., 1..]).to_owned();
    // let _dsq = data.t().dot(data);

    // The least squares regression call:
    let result = data_clone.least_squares(&target_var).unwrap();
    let b: Vec<f64> = result.solution.to_vec();

    b
}

/// Standardise two arrays of (variables, observations) to mutual scales for each variable.
///
/// # Arguments
/// * `values1` - An Array2 array of (variables, observations)
/// * `values2` - Another Array2 array of (variables, observations)
///
/// # Returns
/// A tuple of (Array2<f64>, Array2<f64>) containing transformed versions of both input arrays,
/// where each variable in each array is transformed to the mutual scale defined by mean and
/// standard devisions of all observations of both values1 and values2 for that variable.
pub fn standardise_arrays(values1: &mut Array2<f64>, values2: &mut Array2<f64>) {
    let sum_values1: ndarray::Array1<f64> = values1.axis_iter(Axis(0)).map(|v| v.sum()).collect();
    let sum_values2: ndarray::Array1<f64> = values2.axis_iter(Axis(0)).map(|v| v.sum()).collect();
    let sum_values: ndarray::Array1<f64> = &sum_values1 + &sum_values2;

    let sum_values1_sq: ndarray::Array1<f64> = values1
        .axis_iter(Axis(0))
        .map(|v| v.mapv(|x| x.powi(2)).sum())
        .collect();
    let sum_values2_sq: ndarray::Array1<f64> = values2
        .axis_iter(Axis(0))
        .map(|v| v.mapv(|x| x.powi(2)).sum())
        .collect();
    let sum_values_sq: ndarray::Array1<f64> = &sum_values1_sq + &sum_values2_sq;

    // Calculate standard deviations:
    let nobs = 2.0 * values1.ncols() as f64;
    let mean_vals: ndarray::Array1<f64> = &sum_values / nobs;
    let std_devs: ndarray::Array1<f64> =
        ((&sum_values_sq / nobs) - (&sum_values / nobs).mapv(|x| x.powi(2))).mapv(f64::sqrt);

    // Transform values:
    for (i, (&mean, &std_dev)) in mean_vals.iter().zip(std_devs.iter()).enumerate() {
        values1
            .index_axis_mut(Axis(0), i)
            .mapv_inplace(|x| (x - mean) / std_dev);
        values2
            .index_axis_mut(Axis(0), i)
            .mapv_inplace(|x| (x - mean) / std_dev);
    }
}

/// Adjusts the first row of `values1` based on the multi-linear regression coefficients of the
/// remaining rows of `values1` against `values2`.
///
/// This effectivly removes the dependence of the first row of `values1` by on all other
/// variables/rows, and replaces it with the dependence of `values2` on the same variables.
/// Importantly, this adjustment also standardises the values to a different scale.
///
/// # Arguments
///
/// * `values1` - A 2D array where the first row is the variable to be adjusted and the remaining
/// rows are the other variables.
/// * `values2` - A 2D array with the same structure as `values1`, used to calculate the MLR
/// coefficients for adjustment.
///
/// * Example
/// let mut v1 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [2.1, 3.2, 4.1, 5.2, 5.9]];
/// let v1_orig = v1.clone();
/// let v2 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [3.1, 4.3, 5.3, 6.5, 7.3]];
/// adj_for_beta(&mut v1, &v2);
/// assert_ne!(v1, v1_orig, "v1 should differ from v1_orig");
/// assert_eq!(
///     v1.slice(s![1.., ..]),
///     v1_orig.slice(s![1.., ..]),
///     "Only the first row of v1 should be different"
/// );
pub fn adj_for_beta(values1: &mut Array2<f64>, values2: &Array2<f64>) {
    // Calculate MLR regression coefficients between first variables and all others:
    let beta1 = mlr_beta(values1);
    let beta2 = mlr_beta(values2);
    // Then adjust `values1` by removing its dependence on those variables, and replacing with the
    // dependnece of values2 on same variables:
    let mut result = ndarray::Array1::zeros(values1.ncols());
    for i in 0..values1.ncols() {
        let b1 = ndarray::Array1::from(beta1.clone());
        let b2 = ndarray::Array1::from(beta2.clone());
        let values_slice = values1.slice(s![1.., i]).to_owned();
        let product = &values_slice * (1.0 + &b2 - &b1);
        result[i] = product.sum();
    }
    values1.row_mut(0).assign(&result);
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

    #[test]
    fn test_adj_for_beta() {
        let mut v1 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [2.1, 3.2, 4.1, 5.2, 5.9]];
        let v1_orig = v1.clone();
        let v2 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [3.1, 4.3, 5.3, 6.5, 7.3]];
        adj_for_beta(&mut v1, &v2);
        assert_ne!(
            v1, v1_orig,
            "v1 should be different from v1_orig after adj_for_beta"
        );
        assert_eq!(
            v1.slice(s![1.., ..]),
            v1_orig.slice(s![1.., ..]),
            "Only the first row of v1 should be different"
        );
    }

    #[test]
    fn test_standardise_arrays() {
        let values1 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [2.1, 3.2, 4.1, 5.2, 5.9]];
        let values2 = array![[1.0, 2.0, 3.0, 4.0, 5.0], [3.1, 4.3, 5.3, 6.5, 7.3]];
        let mut values1_std = values1.clone();
        let mut values2_std = values2.clone();

        standardise_arrays(&mut values1_std, &mut values2_std);

        // Standardising should reduce both mean values and standard deviations of all variables.
        for i in 0..values1.nrows() {
            // First test mean values:
            let mean1: f64 = values1.row(i).mean().unwrap();
            let mean2: f64 = values2.row(i).mean().unwrap();
            let mean_std1: f64 = values1_std.row(i).mean().unwrap();
            let mean_std2: f64 = values2_std.row(i).mean().unwrap();
            assert!(mean_std1.abs() < mean1.abs());
            assert!(mean_std2.abs() < mean2.abs());

            // Then standard deviations:
            let sd1: f64 = values1.row(i).std(1.0);
            let sd2: f64 = values2.row(i).std(1.0);
            let sd_std1: f64 = values1_std.row(i).std(1.0);
            let sd_std2: f64 = values2_std.row(i).std(1.0);
            assert!(sd_std1.abs() < sd1.abs());
            assert!(sd_std2.abs() < sd2.abs());
        }
    }
}
