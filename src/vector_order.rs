pub fn order_vectors(vector1: &[Vec<f64>], vector2: &[Vec<f64>]) -> Vec<usize> {
    use rayon::prelude::*;

    let mapping: Vec<usize> = (0..vector1[0].len())
        .into_par_iter()
        .map(|i| {
            let mut min_distance = f64::MAX;
            let mut min_index = 0;
            let point1 = vector1.iter().map(|v| v[i]).collect::<Vec<_>>();

            for (index, _) in vector2[0].iter().enumerate() {
                let point2 = vector2.iter().map(|v| v[index]).collect::<Vec<_>>();
                let distance = squared_euclidean(&point1, &point2);
                if distance < min_distance {
                    min_distance = distance;
                    min_index = index;
                }
            }

            min_index
        })
        .collect();

    mapping
}

fn squared_euclidean(point1: &[f64], point2: &[f64]) -> f64 {
    point1
        .iter()
        .zip(point2.iter())
        .map(|(&x1, &x2)| (x1 - x2).powi(2))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squared_euclidean() {
        let point1 = vec![1.0, 2.0, 3.0];
        let point2 = vec![4.0, 5.0, 6.0];
        let result = squared_euclidean(&point1, &point2);
        assert_eq!(result, 27.0);
    }

    #[test]
    fn test_order_vectors() {
        // Vectors of 2 dimensions with 3 points each. The order should be then the order of the
        // second vector relative to the first.
        let vector1 = vec![vec![3.0, 2.0, 1.0], vec![6.0, 5.0, 4.0]];
        let vector2 = vec![vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0]];
        let result = order_vectors(&vector1, &vector2);
        assert_eq!(result, vec![2, 1, 0]);
    }
}
