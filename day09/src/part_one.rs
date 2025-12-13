use crate::problem::Problem;

pub fn part_one(problem: &Problem) -> u64 {
    let tiles = &problem.tiles;
    let total_tiles = tiles.len();
    let mut biggest_square = 0;
    for i in 0..total_tiles {
        for j in (i + 1)..total_tiles {
            let tile_a = &tiles[i];
            let tile_b = &tiles[j];
            let area = tile_a.compute_area(tile_b);
            if area > biggest_square {
                biggest_square = area;
            }
        }
    }
    biggest_square as u64
}
