extern crate rust_sat_polyomino as rsp;

use rsp::grid_mask_str2bool;
use rsp::solve;

fn main() {
    let pieces = vec![vec![(0, 0), (0, 1), (0, 2)], vec![(0, 0), (0, 1), (1, 1)]];
    let grid_mask_str = vec!["XXX", "XX.", "X.."];
    let grid_mask = grid_mask_str2bool(grid_mask_str);
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
