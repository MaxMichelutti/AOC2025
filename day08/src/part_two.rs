use crate::problem::Problem;
use disjoint::DisjointSet;
use std::collections::BTreeMap;

pub fn part_two(problem: &Problem) -> u64 {
    // Implement the logic for part one here
    let mut mfset = DisjointSet::with_len(problem.size());

    // compute distance matrix so we don't recompute distances at every iteration
    let mut distance_matrix = vec![vec![0; problem.size()]; problem.size()];
    for i in 0..problem.size() {
        for j in i + 1..problem.size() {
            let distance = problem.junction_boxes[i].distance_to(&problem.junction_boxes[j]);
            distance_matrix[i][j] = distance;
        }
    }

    // sort all possible pairs by distance using a tree
    let mut sorted_pairs = BTreeMap::new();
    for i in 0..problem.size() {
        for j in i + 1..problem.size() {
            sorted_pairs.insert(distance_matrix[i][j], (i, j));
        }
    }

    let mut last_connection = (0, 0);
    for (_, (i, j)) in sorted_pairs {
        // merge the indexes in same set
        if !mfset.is_joined(i, j) {
            mfset.join(i, j);
            last_connection = (i, j);
        }
    }

    // println!(
    //     "Last connection made between {} and {} with distance {}. {:?} --- {:?}",
    //     last_connection.0,
    //     last_connection.1,
    //     distance_matrix[last_connection.0][last_connection.1],
    //     problem.junction_boxes[last_connection.0],
    //     problem.junction_boxes[last_connection.1]
    // );
    let first = &problem.junction_boxes[last_connection.0];
    let second = &problem.junction_boxes[last_connection.1];
    (first.x * second.x) as u64
}
