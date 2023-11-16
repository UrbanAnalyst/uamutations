pub fn order_vectors_kd(vector1: &[Vec<f64>], vector2: &[Vec<f64>]) -> Vec<usize> {
    use kdtree::distance::squared_euclidean;
    use kdtree::KdTree;
    use std::collections::HashSet;

    let mut kdtree = KdTree::new(vector1[0].len());
    for (i, point) in vector2.iter().enumerate() {
        kdtree.add(point, i).unwrap();
    }

    let mut used_indices = HashSet::new();
    let mut mapping = Vec::new();

    for i in 0..vector1[0].len() {
        let point1 = vector1.iter().map(|v| v[i]).collect::<Vec<_>>();
        let mut min_distance = f64::MAX;
        let mut min_index = 0;

        for j in 0..vector2[0].len() {
            if used_indices.contains(&j) {
                continue;
            }

            let point2 = vector2.iter().map(|v| v[j]).collect::<Vec<_>>();
            let distance = squared_euclidean(&point1, &point2);

            if distance < min_distance {
                min_distance = distance;
                min_index = j;
            }
        }

        used_indices.insert(min_index);
        mapping.push(min_index);
    }

    mapping
}

pub fn order_vectors(vector1: &[Vec<f64>], vector2: &[Vec<f64>]) -> Vec<usize> {
    use std::collections::HashSet;

    let mut used_indices = HashSet::new();
    let mut mapping = Vec::new();

    for i in 0..vector1[0].len() {
        let mut min_distance = f64::MAX;
        let mut min_index = 0;
        let point1 = vector1.iter().map(|v| v[i]).collect::<Vec<_>>();

        for (index, _) in vector2[0].iter().enumerate() {
            if used_indices.contains(&index) {
                continue;
            }

            let point2 = vector2.iter().map(|v| v[index]).collect::<Vec<_>>();
            let distance = squared_euclidean(&point1, &point2);
            if distance < min_distance {
                min_distance = distance;
                min_index = index;
            }
        }

        used_indices.insert(min_index);
        mapping.push(min_index);
    }

    mapping
}

fn squared_euclidean(point1: &[f64], point2: &[f64]) -> f64 {
    point1
        .iter()
        .zip(point2.iter())
        .map(|(&x1, &x2)| (x1 - x2).powi(2))
        .sum()
}

use ndarray::{Array2, Axis};
use ndarray_linalg::{Eigh, UPLO};

pub fn pca(vector1: &[Vec<f64>]) -> Array2<f64> {
    let rows = vector1.len(); // multivariate dimensions
    let cols = vector1[0].len(); // nobs
    let flattened: Vec<f64> = vector1.iter().flatten().cloned().collect();
    let data = Array2::from_shape_vec((rows, cols), flattened).unwrap();

    // Calculate the mean of the data
    let mean = data.mean_axis(Axis(0)).unwrap();

    // Subtract the mean from the data
    let centered_data = data.clone() - &mean;

    // Calculate the covariance matrix of the centered data
    let cov = centered_data.t().dot(&centered_data) / (data.len_of(Axis(0)) - 1) as f64;

    // Calculate the eigenvalues and eigenvectors of the covariance matrix
    let (eigenvalues, eigenvectors) = cov.eigh(UPLO::Lower).unwrap();

    // Sort the eigenvalues and corresponding eigenvectors in descending order
    let mut sorted_indices = eigenvalues
        .indexed_iter()
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    sorted_indices.sort_unstable_by(|&i, &j| eigenvalues[j].partial_cmp(&eigenvalues[i]).unwrap());

    // let first_eigenvector = eigenvectors.select(Axis(0), &sorted_indices[0..1]);
    // let projected_data = centered_data.dot(&first_eigenvector.t());

    eigenvectors.select(Axis(0), &sorted_indices[0..1])
}
