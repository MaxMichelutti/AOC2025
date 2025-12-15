use crate::part_one::solve;
use crate::problem::Problem;

pub fn part_two(problem: &Problem) -> u64 {
    // Implement the logic for part two here
    let reverse = problem.reverse();
    let solution_1 = solve_dac_fft(problem, &reverse);
    let solution_2 = solve_fft_dac(problem, &reverse);
    // one is guaranteed to be 0 as long as the problem does not contain cycles
    // if the problem contains relevant cycles, no solution exists
    std::cmp::max(solution_1, solution_2)
}

fn solve_dac_fft(problem: &Problem, reverse: &Problem) -> u64 {
    let srv_to_dac = solve(problem, reverse, problem.svr_id, problem.dac_id);
    let dac_to_fft = solve(problem, reverse, problem.dac_id, problem.fft_id);
    let fft_to_out = solve(problem, reverse, problem.fft_id, problem.out_id);
    srv_to_dac * dac_to_fft * fft_to_out
}

fn solve_fft_dac(problem: &Problem, reverse: &Problem) -> u64 {
    let srv_to_fft = solve(problem, reverse, problem.svr_id, problem.fft_id);
    let fft_to_dac = solve(problem, reverse, problem.fft_id, problem.dac_id);
    let dac_to_out = solve(problem, reverse, problem.dac_id, problem.out_id);
    srv_to_fft * fft_to_dac * dac_to_out
}
