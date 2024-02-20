use nalgebra::{DMatrix, DVector};

/// Convert values for selected columns of an input variable to logarithmic scales.
///
/// # Arguments
///
/// * `values` - Matrix of values to be transformed.
///
/// # Returns
///
/// Matrix of (potentially) transformed values.
///
/// # Panics
///
/// This function will panic if `values` is empty.
///
/// # Example
/// ```
/// use nalgebra::DMatrix;
/// use uamutations::utils::log_transform;
/// let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
/// let mut values = DMatrix::from_vec(3, 2, values);
/// let log_scale = log_transform(&mut values, &vec!["parking".to_string(), "school_dist".to_string()]);
/// assert_eq!(log_scale, true);
/// ```
pub fn log_transform(values: &mut DMatrix<f64>, varnames: &[String]) -> bool {
    assert!(!values.is_empty(), "values must not be empty");

    // These selected variables to be log-transformed are specified in uaengine/R/ua-export.R:
    let log_vars = ["parking", "school_dist", "intervals"];
    let epsilon = -10.; // Small constant of log10(1e-10) to avoid NaN from log(<= 0)

    let mut log_scale = false;
    for (i, var) in varnames.iter().enumerate() {
        if log_vars.contains(&var.as_str()) {
            if i == 0 {
                log_scale = true;
            }
            values
                .column_mut(i)
                .iter_mut()
                .for_each(|x| *x = if *x > 0.0 { x.log10() } else { epsilon });
        }
    }

    log_scale
}

/// Calcualte mean and standard deviation of first column of input `values` in DMatrix format.
///
/// # Arguments
///
/// * `values` - An Array2 object from which mean and SD values are calcualted from the first
/// column.
///
/// # Panics
///
/// This function will panic if `values` is empty.
///
/// # Returns
///
/// Tuple of mean and standard deviation.
///
/// # Example
/// ```
/// use nalgebra::DMatrix;
/// use uamutations::utils::mean_sd_dmat;
/// let data = DMatrix::from_vec(5, 1, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
/// let result = mean_sd_dmat(&data);
/// assert_eq!(result.0, 3.0);
/// assert_eq!(result.1, 1.5811388300841898);
/// ```
pub fn mean_sd_dmat(values: &DMatrix<f64>) -> (f64, f64) {
    assert!(!values.is_empty(), "values must not be empty");

    let sum_vals: f64 = values.column(0).sum();
    let sum_vals_sq: f64 = values.column(0).iter().map(|&x| x.powi(2)).sum();
    let nobs = values.nrows() as f64;
    let mean: f64 = sum_vals / nobs;
    let sd: f64 = ((sum_vals_sq / nobs - (sum_vals / nobs).powi(2)) * nobs / (nobs - 1.0)).sqrt();

    (mean, sd)
}

/// Calculate mean and standard deviation of a column vector.
///
/// # Arguments
///
/// * `column` - A column vector from which mean and SD values are calculated.
///
/// # Panics
///
/// This function will panic if `column` is empty.
///
/// # Returns
///
/// Tuple of mean and standard deviation.
pub fn mean_sd_column(column: &DVector<f64>) -> (f64, f64) {
    assert!(!column.is_empty(), "column must not be empty");

    let sum_vals: f64 = column.sum();
    let sum_vals_sq: f64 = column.iter().map(|&x| x.powi(2)).sum();
    let nobs = column.len() as f64;
    let mean: f64 = sum_vals / nobs;
    let sd: f64 = ((sum_vals_sq / nobs - (sum_vals / nobs).powi(2)) * nobs / (nobs - 1.0)).sqrt();

    (mean, sd)
}
