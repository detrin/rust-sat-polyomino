use std::time::Instant;
use std::vec;

use varisat::lit::Lit;
use varisat::solver::Solver;
use varisat::ExtendFormula;

#[derive(Debug, Clone, Eq)]
struct Piece {
    cells: Vec<(usize, usize)>,
}

impl Ord for Piece {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ((min_x, min_y), _) = self.get_min_max();
        let ((other_min_x, other_min_y), _) = other.get_min_max();
        if min_x != other_min_x {
            return min_x.cmp(&other_min_x);
        }
        if min_y != other_min_y {
            return min_y.cmp(&other_min_y);
        }
        let mut self_cells = self.cells.clone();
        let mut other_cells = other.cells.clone();
        self_cells.sort();
        other_cells.sort();
        for (x, y) in self_cells.iter().zip(other_cells.iter()) {
            if x.0 != y.0 {
                return x.0.cmp(&y.0);
            }
            if x.1 != y.1 {
                return x.1.cmp(&y.1);
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        let ((min_x, min_y), _) = self.get_min_max();
        let ((other_min_x, other_min_y), _) = other.get_min_max();
        if min_x != other_min_x || min_y != other_min_y {
            return false;
        }
        self.cells == other.cells
    }
}

impl Piece {
    fn new(cells: Vec<(usize, usize)>) -> Piece {
        Piece { cells }
    }

    fn get_min_max(&self) -> ((usize, usize), (usize, usize)) {
        let mut min_x = self.cells[0].0;
        let mut min_y = self.cells[0].1;
        let mut max_x = self.cells[0].0;
        let mut max_y = self.cells[0].1;
        for (x, y) in self.cells.iter() {
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        ((min_x, min_y), (max_x, max_y))
    }

    fn shift_to_origin(&mut self) {
        let ((min_x, min_y), _) = self.get_min_max();
        for (cell_x, cell_y) in self.cells.iter_mut() {
            *cell_x -= min_x;
            *cell_y -= min_y;
        }
    }

    fn shift(&mut self, x: usize, y: usize) {
        for (cell_x, cell_y) in self.cells.iter_mut() {
            *cell_x += x;
            *cell_y += y;
        }
    }

    fn rotate90(&mut self) {
        let (_, (_, max_y)) = self.get_min_max();
        for (cell_x, cell_y) in self.cells.iter_mut() {
            *cell_y = max_y - *cell_y;
            std::mem::swap(cell_x, cell_y);
        }
        self.shift_to_origin();
    }

    fn flip_x(&mut self) {
        let (_, (max_x, _)) = self.get_min_max();
        for (cell_x, _) in self.cells.iter_mut() {
            *cell_x = max_x - *cell_x;
        }
        self.shift_to_origin();
    }

    fn flip_y(&mut self) {
        let (_, (_, max_y)) = self.get_min_max();
        for (_, cell_y) in self.cells.iter_mut() {
            *cell_y = max_y - *cell_y;
        }
        self.shift_to_origin();
    }

    #[allow(dead_code)]
    fn print(&self) {
        let ((min_x, min_y), (max_x, max_y)) = self.get_min_max();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.cells.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_with_grid(&self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                if self.cells.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn generate_all_orientations(&self, flips_allowed: bool) -> Vec<Piece> {
        let mut pieces = Vec::new();
        let mut piece = self.clone();
        for _ in 0..4 {
            pieces.push(piece.clone());
            piece.rotate90();
        }
        if flips_allowed {
            piece.flip_x();
            for _ in 0..4 {
                pieces.push(piece.clone());
                piece.rotate90();
            }
            piece.flip_x();
            piece.flip_y();
            for _ in 0..4 {
                pieces.push(piece.clone());
                piece.rotate90();
            }
            piece.flip_x();
            for _ in 0..4 {
                pieces.push(piece.clone());
                piece.rotate90();
            }
        }

        pieces.sort();
        pieces.dedup();
        pieces
    }

    fn generate_all_positions(&self, grid_mask: Vec<Vec<bool>>) -> Vec<Piece> {
        let height = grid_mask.len();
        let width = grid_mask[0].len();
        let mut pieces = Vec::new();
        let ((_, _), (max_x, max_y)) = self.get_min_max();
        for x in 0..(width - max_x) {
            for y in 0..(height - max_y) {
                let mut piece = self.clone();
                piece.shift_to_origin();
                piece.shift(x, y);
                let (_, (max_x, max_y)) = piece.get_min_max();
                if max_x >= width || max_y >= height {
                    continue;
                }
                let mut all_true = true;
                for (cell_x, cell_y) in piece.cells.iter() {
                    if !grid_mask[*cell_y][*cell_x] {
                        all_true = false;
                        break;
                    }
                }
                if all_true {
                    pieces.push(piece);
                }
            }
        }
        pieces
    }
}

fn generate_lit_grid(width: usize, height: usize, last_lit_num: usize) -> Vec<Vec<Lit>> {
    let mut pos = last_lit_num;
    let mut grid = vec![];
    for _ in 0..width {
        let mut row = vec![];
        for _ in 0..height {
            row.push(varisat::Lit::from_dimacs(pos as isize));
            pos += 1;
        }
        grid.push(row);
    }
    grid
}

pub fn grid_mask_str2bool(grid_mask_str: Vec<&str>) -> Vec<Vec<bool>> {
    let mut grid_mask = vec![];
    for row in grid_mask_str {
        let mut grid_mask_row = vec![];
        for c in row.chars() {
            if c == 'X' {
                grid_mask_row.push(true);
            } else {
                grid_mask_row.push(false);
            }
        }
        grid_mask.push(grid_mask_row);
    }
    grid_mask
}

pub fn solve(
    pieces: Vec<Vec<(usize, usize)>>,
    grid_mask: Vec<Vec<bool>>,
    flips_allowed: Option<bool>,
    verbose: Option<bool>,
) -> Result<Vec<Vec<(usize, usize)>>, String> {
    let is_verbose = verbose.unwrap_or(false);
    let is_flips_allowed = flips_allowed.unwrap_or(false);
    let mut solver = Solver::new();
    let grid_height = grid_mask.len();
    let grid_width = grid_mask[0].len();

    let mut lit_pos = 1;
    let mut clause_pos = 1;

    let mut pieces_prep = vec![];
    for mut piece in pieces {
        piece.sort_unstable();
        pieces_prep.push(Piece::new(piece));
    }

    let mut grid_lit = vec![];
    let mut pieces_orientations_lit_num = vec![];
    let time_now = Instant::now();
    for piece in pieces_prep {
        // println!("piece: {:?}", piece);
        let mut piece_ord_lit = vec![];
        let mut piece_ord_lit_num = vec![];
        let pieces_oriented = piece.generate_all_orientations(is_flips_allowed);
        for p_oriented in pieces_oriented {
            for p_positioned in p_oriented.generate_all_positions(grid_mask.clone()).iter() {
                // p_positioned.print_with_grid(grid_width, grid_height);

                let piece_grid_lit = generate_lit_grid(grid_width, grid_height, lit_pos);
                lit_pos += grid_height * grid_width;
                let piece_oriented_lit = varisat::Lit::from_dimacs(lit_pos as isize);
                piece_ord_lit_num.push(lit_pos);

                for (x, y) in p_positioned.cells.iter() {
                    solver.add_clause(&[!piece_oriented_lit, piece_grid_lit[*x][*y]]);
                    clause_pos += 1;
                }

                lit_pos += 1;
                piece_ord_lit.push(piece_oriented_lit);
                grid_lit.push(piece_grid_lit);
            }
        }
        solver.add_clause(&piece_ord_lit);
        clause_pos += 1;

        for i in 0..piece_ord_lit.len() {
            for j in (i + 1)..piece_ord_lit.len() {
                solver.add_clause(&[!piece_ord_lit[i], !piece_ord_lit[j]]);
                clause_pos += 1;
            }
        }

        pieces_orientations_lit_num.push(piece_ord_lit_num);
    }
    let time_elapsed = time_now.elapsed();
    if is_verbose {
        println!("time elapsed: {:.2?}", time_elapsed);
        println!("{} vars", lit_pos);
    }

    let time_now = Instant::now();
    for x in 0..grid_width {
        for y in 0..grid_height {
            if !grid_mask[y][x] {
                continue;
            }
            // println!("({},{}) ", x, y);
            let mut clause = vec![];
            for depth in 0..grid_lit.len() {
                clause.push(grid_lit[depth][x][y]);
            }
            solver.add_clause(&clause);
            clause_pos += 1;

            for depth1 in 0..grid_lit.len() {
                for depth2 in (depth1 + 1)..grid_lit.len() {
                    solver.add_clause(&[!grid_lit[depth1][x][y], !grid_lit[depth2][x][y]]);
                    clause_pos += 1;
                }
            }
        }
    }
    let time_elapsed = time_now.elapsed();
    if is_verbose {
        println!("time elapsed: {:.2?}", time_elapsed);
        println!("{} clauses", clause_pos);
    }

    let time_now = Instant::now();
    let result = solver.solve();
    let time_elapsed = time_now.elapsed();
    if is_verbose {
        println!("time elapsed: {:.2?}", time_elapsed);
        println!("Result: {:?}", result);
    }

    if result.is_ok() {
        let mut pieces_positioned = vec![];
        let time_now = Instant::now();
        let model = solver.model().unwrap();
        let mut cells_to_print = vec![];
        for _ in 0..grid_width {
            let mut row = vec![];
            for _ in 0..grid_height {
                row.push(' ');
            }
            cells_to_print.push(row);
        }
        let mut curr_letter = 'A';
        for depth in 0..grid_lit.len() {
            let mut piece_positioned = vec![];
            let mut if_found = false;
            for x in 0..grid_width {
                for y in 0..grid_height {
                    if !grid_mask[y][x] {
                        continue;
                    }
                    if model.contains(&grid_lit[depth][x][y]) {
                        cells_to_print[x][y] = curr_letter;
                        piece_positioned.push((x, y));
                        if_found = true;
                    }
                }
            }
            if if_found {
                curr_letter = (curr_letter as u8 + 1) as char;
                pieces_positioned.push(piece_positioned.clone());
            }
        }
        let time_elapsed = time_now.elapsed();
        println!("time elapsed: {:.2?}", time_elapsed);
        for row in cells_to_print {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
        return Ok(pieces_positioned);
    }

    Err("No solution found".to_string())
}
