extern crate rust_sat_polyomino as rsp;

use rsp::solve;

fn main() {
    let pieces = vec![
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)], // A
        vec![(2, 0), (2, 1), (1, 1), (0, 1), (0, 2)], // B
        vec![(0, 0), (0, 1), (1, 1), (0, 2), (0, 3)], // C
        vec![(0, 0), (1, 0), (1, 1), (1, 2), (0, 2)], // D
        vec![(2, 0), (2, 1), (1, 1), (1, 2), (0, 1)], // E
        vec![(0, 0), (1, 0), (1, 1), (1, 2), (1, 3)], // F
        vec![(1, 0), (1, 1), (0, 1), (2, 1), (1, 2)], // G
        vec![(1, 0), (1, 1), (0, 1), (0, 2), (0, 3)], // H
        vec![(1, 0), (0, 1), (1, 1), (0, 2), (1, 2)], // I
        vec![(2, 0), (2, 1), (1, 1), (1, 2), (0, 2)], // J
        vec![(2, 0), (2, 1), (1, 1), (0, 1), (2, 2)], // K
        vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)], // L
    ];
    // create grid_mask with 6x10 trues
    let grid_mask = vec![vec![true; 10]; 6];
    let flips_allowed = Some(true);
    let verbose = Some(true);
    let result = solve(pieces, grid_mask, flips_allowed, verbose);
    match result {
        Ok(pieces_positioned) => {
            println!("Pieces positioned: {:?}", pieces_positioned);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
