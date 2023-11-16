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
