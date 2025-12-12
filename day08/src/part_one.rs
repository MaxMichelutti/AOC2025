use crate::problem::Problem;
use disjoint::DisjointSet;
use std::collections::BTreeMap;
use std::collections::HashSet;

pub fn part_one(problem: &Problem) -> u64 {
    // Implement the logic for part one here
    let mut mfset = DisjointSet::with_len(problem.size());

    // I don't like this, but I have to force checkk if problem is example given
    let limit = if problem.size() == 20 { 10 } else { 1000 };

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

    // iterate connections
    let mut count = 0;
    for (_, (i, j)) in sorted_pairs {
        // stop at limit iterations
        count += 1;
        if count > limit {
            break;
        }
        //println!("Merging {} and {} with distance {}: {:?} --- {:?}", i, j, distance_matrix[i][j], problem.junction_boxes[i], problem.junction_boxes[j]);
        // merge the indexes in same set
        mfset.join(i, j);
    }

    // collect distinct set sizes
    let mut set_sizes = HashSet::new();
    for s in mfset.sets() {
        set_sizes.insert(s.len() as u64);
    }

    // sort distinct set sizes from biggest to smallest
    let mut sorted = set_sizes.iter().cloned().collect::<Vec<u64>>();
    sorted.sort_by(|a, b| b.cmp(a));

    // multiply 3 biggest sizes
    let mut acc = 1u64;
    for item in sorted.iter().take(3) {
        acc *= *item;
    }
    acc
}
