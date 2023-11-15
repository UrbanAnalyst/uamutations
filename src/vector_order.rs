pub fn order_vectors_kd(vector1: &[Vec<f64>], vector2: &[Vec<f64>]) -> Vec<usize> {
    use kdtree::distance::squared_euclidean;
    use kdtree::KdTree;
    use std::collections::HashSet;

    let mut kdtree = KdTree::new(vector2[0].len());
    for (i, point) in vector2.iter().enumerate() {
        kdtree.add(point, i).unwrap();
    }

    let mut used_indices = HashSet::new();
    let mut mapping = Vec::new();

    for point1 in vector1 {
        let mut i = 0;
        loop {
            let nearest = kdtree.nearest(point1, i + 1, &squared_euclidean).unwrap();
            let index = *nearest[i].1;
            if !used_indices.contains(&index) {
                used_indices.insert(index);
                mapping.push(index);
                break;
            }
            i += 1;
        }
    }

    mapping
}

use ndarray::Array1;
use ndarray::Array2;
use pca::PCA;

pub fn pca(vector1: &[Vec<f64>]) -> Array1<f64> {
    let mut pca = PCA::new();

    let rows = vector1.len(); // multivariate dimensions
    let cols = vector1[0].len(); // nobs
    let flattened: Vec<f64> = vector1.iter().flatten().cloned().collect();
    let array = Array2::from_shape_vec((rows, cols), flattened).unwrap();
    // Array needs to be transposed for pca:
    let array = array.t().to_owned();

    pca.fit(array.clone(), None).unwrap();

    let transformed = pca.transform(array).unwrap();
    let first_pca_axis = transformed.column(0).to_owned();
    first_pca_axis
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
