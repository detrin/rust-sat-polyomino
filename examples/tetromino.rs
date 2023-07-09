extern crate rust_sat_polyomino as rsp;

use rsp::solve;
use std::vec;

fn main() {
    let mut pieces = vec![
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 1)],
        vec![(1, 0), (1, 1), (0, 1), (0, 2)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2)],
    ];
    pieces.extend(pieces.clone());
    // make the mask with  5 x 4 of trues
    let grid_mask = vec![vec![true; 8]; 5];
    let flips_allowed = Some(false);
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
