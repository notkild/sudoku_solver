

pub type Grid = [[u8; 9]; 9];

pub fn solve(grid: &Grid) -> Grid {
    let mut grid_clone = grid.clone();
    let empty = get_empty(&grid_clone);
    let mut idx = 0;
    fill_cell(&mut grid_clone, &mut idx, &empty);
    grid_clone
}

fn get_empty(grid: &Grid) -> Vec<(usize, usize)> {
    let mut empty = Vec::new();
    for y in 0..9 {
        for x in 0..9 {
            if grid[y][x] == 0 {
                empty.push((x, y));
            }
        }
    }
    empty
}

fn fill_cell(grid: &mut Grid, idx: &mut usize, empty: &Vec<(usize, usize)>) {
    if *idx >= empty.len() {
        return;
    }
    let (x, y) = empty[*idx];
    let mut val = grid[y][x] + 1;
    if val >= 10 {
        grid[y][x] = 0;
        *idx -= 1;
        fill_cell(grid, idx, empty);
        return;
    }

    while !is_valid(grid, val, x, y) {
        if val >= 9 {
            if *idx == 0 {
                return;
            }
            *idx -= 1;
            grid[y][x] = 0;
            fill_cell(grid, idx, empty);
            return;
        }
        val += 1;
    }
    grid[y][x] = val;
    *idx += 1;
    fill_cell(grid, idx, empty);
}

fn is_valid(grid: &Grid, val: u8, x: usize, y: usize) -> bool {
    if !check_row(grid, &val, &y) {
        return false;
    }
    if !check_column(grid, &val, &x) {
        return false;
    }
    if !check_box(grid, &val, &x, &y) {
        return false;
    }
    true
}

fn check_row(grid: &Grid, val: &u8, y: &usize) -> bool {
    for x in 0..9 {
        if grid[*y][x] == *val {
            return false;
        }
    }
    true
}

fn check_column(grid: &Grid, val: &u8, x: &usize) -> bool {
    for y in 0..9 {
        if grid[y][*x] == *val {
            return false;
        }
    }
    true
}

fn check_box(grid: &Grid, val: &u8, x: &usize, y: &usize) -> bool {
    let x_box = x / 3 * 3;
    let y_box = y / 3 * 3;
    for y in 0..3 {
        for x in 0..3 {
            let x = x + x_box;
            let y = y + y_box;
            if grid[y][x] == *val {
                return false;
            }
        }
    }
    true
}

pub fn print_grid(grid: &Grid) {
    for y in 0..9 {
        for x in 0..9 {
            let val = grid[y][x];
            print!("|");
            match val {
                0 => print!(" "),
                x => print!("{}", x),
            }
        }
        println!("|");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const GRID: Grid = [[0, 1, 0, 4, 0, 2, 0, 5, 0],
                        [5, 0, 0, 0, 0, 0, 0, 0, 6],
                        [0, 0, 0, 3, 0, 1, 0, 0, 0],
                        [7, 0, 5, 0, 0, 0, 4, 0, 8],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [2, 0, 8, 0, 0, 0, 5, 0, 9],
                        [0, 0, 0, 9, 0, 6, 0, 0, 0],
                        [6, 0, 0, 0, 0, 0, 0, 0, 2],
                        [0, 7, 0, 1, 0, 3, 0, 4, 0]];


    const GRID_SOLVED: Grid = [[3, 1, 9, 4, 6, 2, 8, 5, 7],
                               [5, 2, 4, 7, 9, 8, 1, 3, 6],
                               [8, 6, 7, 3, 5, 1, 9, 2, 4],
                               [7, 3, 5, 2, 1, 9, 4, 6, 8],
                               [1, 9, 6, 8, 4, 5, 2, 7, 3],
                               [2, 4, 8, 6, 3, 7, 5, 1, 9],
                               [4, 5, 3, 9, 2, 6, 7, 8, 1],
                               [6, 8, 1, 5, 7, 4, 3, 9, 2],
                               [9, 7, 2, 1, 8, 3, 6, 4, 5]];
    #[test]
    fn solve_test() {
        let grid = solve(&GRID);
        println!("");
        print_grid(&grid);
        assert_eq!(grid, GRID_SOLVED);
    }
}
