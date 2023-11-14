pub fn order_vectors(vector1: &Vec<Vec<f64>>, vector2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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
                mapping.push(vector2[index].clone());
                break;
            }
            i += 1;
        }
    }

    // The assignment is a vector of indices where assignment[i] is the index in vector2 of the point assigned to the point at index i in vector1
    println!("{:?}", mapping);

    mapping
}
