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

pub fn order_vectors(vector1: &[Vec<f64>], vector2: &[Vec<f64>]) -> Vec<usize> {
    use std::collections::HashSet;

    let mut used_indices = HashSet::new();
    let mut mapping = Vec::new();

    for point1 in vector1 {
        let mut min_distance = f64::MAX;
        let mut min_index = 0;

        for (index, point2) in vector2.iter().enumerate() {
            if used_indices.contains(&index) {
                continue;
            }

            let distance = squared_euclidean(point1, point2);
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
