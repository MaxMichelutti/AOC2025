use crate::problem::Problem;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(problem: &Problem) -> u64 {
    // println!("Problem: {:?}", problem);
    let reverse = problem.reverse();
    // println!("Reversed Problem: {:?}", reverse);
    // println!("Ids: {:?}", problem.id_mappings);
    solve(problem, &reverse, problem.you_id, problem.out_id)
}

pub fn solve(problem: &Problem, reverse_problem: &Problem, start: u64, end: u64) -> u64 {
    // Implement the logic for part one here
    if start == 0 || end == 0 {
        // println!("Missing 'start' or 'end' node");
        return 0;
    }
    // println!("Problem ids: {:?}", problem.id_mappings);
    let nodes_reachable_from_end = find_reachable_nodes(reverse_problem, end);
    // println!("Nodes reachable from end: {:?}", nodes_reachable_from_end);
    let nodes_reachable_from_start = find_reachable_nodes(problem, start);
    // println!("Nodes reachable from start: {:?}", nodes_reachable_from_start);
    let reachable_nodes = nodes_reachable_from_start
        .intersection(&nodes_reachable_from_end)
        .cloned()
        .collect::<HashSet<u64>>();
    if !reachable_nodes.contains(&start) || !reachable_nodes.contains(&end) {
        // println!("No path from start to end");
        return 0;
    }
    //println!("Good nodes: {:?}", good_nodes);
    let mut reachable_nodes_entering_edges = HashMap::new();
    for &node_id in &reachable_nodes {
        let mut count = 0;
        for neighbour_id in reverse_problem
            .nodes
            .get(&node_id)
            .unwrap()
            .neighbours
            .iter()
        {
            if reachable_nodes.contains(neighbour_id) {
                count += 1;
            }
        }
        reachable_nodes_entering_edges.insert(node_id, count);
    }
    // println!("Good enters: {:?}", reachable_nodes_entering_edges);
    compute_bfs_solution(
        problem,
        start,
        end,
        &reachable_nodes,
        &mut reachable_nodes_entering_edges,
    )
}

fn find_reachable_nodes(problem: &Problem, start: u64) -> HashSet<u64> {
    let mut reachables = HashSet::new();
    find_reachable_nodes_rec(problem, start, &mut reachables);
    reachables
}

fn find_reachable_nodes_rec(
    problem: &Problem,
    current_node_id: u64,
    reachables: &mut HashSet<u64>,
) {
    if reachables.contains(&current_node_id) {
        return;
    }
    reachables.insert(current_node_id);
    for neighbour in &problem.nodes.get(&current_node_id).unwrap().neighbours {
        find_reachable_nodes_rec(problem, *neighbour, reachables);
    }
}

fn compute_bfs_solution(
    problem: &Problem,
    start: u64,
    end: u64,
    reachable_nodes: &HashSet<u64>,
    reachable_nodes_entering_edges: &mut HashMap<u64, u64>,
) -> u64 {
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();
    for node in reachable_nodes {
        paths.insert(node, 0u64);
    }
    *(paths.get_mut(&start).unwrap()) = 1;
    queue.push_back(start);
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        for neighbour_id in &problem.nodes[&current_node].neighbours {
            if reachable_nodes.contains(neighbour_id) {
                *(paths.get_mut(neighbour_id).unwrap()) += paths[&current_node];
                (*reachable_nodes_entering_edges
                    .get_mut(neighbour_id)
                    .unwrap()) -= 1;
                if reachable_nodes_entering_edges[neighbour_id] == 0 {
                    queue.push_back(*neighbour_id);
                }
            }
        }
    }
    //println!("Paths: {:?}", paths);
    paths[&end]
}
