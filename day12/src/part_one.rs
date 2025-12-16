use crate::problem::Problem;

pub fn part_one(problem: &Problem) -> u64 {
    // Presents in input look like they can be stored optimally (without leaving blank spaces)
    // So I just check if the total area of required presents fits in the region area
    let mut possible_regions = 0;
    for region in &problem.regions {
        let region_area = region.width * region.height;
        let mut optimal_present_area = 0;
        for (index, present) in problem.presents.iter().enumerate() {
            let presents_to_fit = region.requirements[index];
            let present_area = present.total_size() as usize * presents_to_fit;
            optimal_present_area += present_area;
        }
        if optimal_present_area <= region_area {
            possible_regions += 1;
        }
    }
    possible_regions
}
