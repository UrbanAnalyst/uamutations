pub fn order_vectors(vector1: &[Vec<f64>], vector2: &[Vec<f64>]) -> Vec<usize> {
    use kdtree::distance::squared_euclidean;
    use kdtree::KdTree;
    use std::collections::HashSet;

    let mut kdtree = KdTree::new(3);
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
