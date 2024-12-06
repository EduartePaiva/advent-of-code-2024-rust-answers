// question 6

use std::collections::HashSet;

pub fn question06(input: &str) -> i64 {
    // I'll try simulation first
    let matrix: Vec<_> = input.lines().map(|v| v.as_bytes()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut visit: HashSet<(usize, usize)> = HashSet::new();

    // up, right, down, left
    // const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut guard_pos = (0, 0);
    // find the guard position
    'outer: for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == b'^' {
                guard_pos = (r, c);
                break 'outer;
            }
        }
    }
    visit.insert(guard_pos);

    loop {
        // going up
        let (row, col) = guard_pos;
        for r in (0..row).rev() {
            if matrix[r][col] == b'#' {
                break;
            }
            guard_pos = (r, col);
            visit.insert(guard_pos);
        }
        if guard_pos.0 == 0 {
            break;
        }
        // going right
        let (row, col) = guard_pos;
        for c in col..cols {
            if matrix[row][c] == b'#' {
                break;
            }
            guard_pos = (row, c);
            visit.insert(guard_pos);
        }
        if guard_pos.1 == cols - 1 {
            break;
        }
        //going down
        let (row, col) = guard_pos;
        for r in row..rows {
            if matrix[r][col] == b'#' {
                break;
            }
            guard_pos.0 = r;
            visit.insert(guard_pos);
        }
        if guard_pos.0 == rows - 1 {
            break;
        }
        //going left
        let (row, col) = guard_pos;
        for c in (0..col).rev() {
            if matrix[row][c] == b'#' {
                break;
            }
            guard_pos.1 = c;
            visit.insert(guard_pos);
        }
        if guard_pos.1 == 0 {
            break;
        }
    }

    visit.len() as i64
}
pub fn question06_part_2(input: &str) -> i64 {
    let mut matrix: Vec<Vec<u8>> = input.lines().map(|v| v.bytes().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut initial_pos = (0, 0);
    // find the guard position
    'outer: for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == b'^' {
                initial_pos = (r, c);
                break 'outer;
            }
        }
    }

    fn detect_cycle(matrix: &Vec<Vec<u8>>, mut guard_pos: (usize, usize)) -> bool {
        #[derive(Eq, PartialEq, Hash)]
        enum Dir {
            UP,
            RIGHT,
            DOWN,
            LEFT,
        }
        let mut reached: HashSet<(usize, usize, Dir)> = HashSet::new();
        let rows = matrix.len();
        let cols = matrix[0].len();

        loop {
            // going up
            let (row, col) = guard_pos;
            for r in (0..row).rev() {
                if matrix[r][col] == b'#' {
                    break;
                }
                guard_pos = (r, col);
            }
            let new_reached = (guard_pos.0, guard_pos.1, Dir::UP);
            if guard_pos.0 == 0 {
                break;
            }
            if reached.contains(&new_reached) {
                return true;
            }
            reached.insert(new_reached);
            // going right
            let (row, col) = guard_pos;
            for c in col..cols {
                if matrix[row][c] == b'#' {
                    break;
                }
                guard_pos = (row, c);
            }
            let new_reached = (guard_pos.0, guard_pos.1, Dir::RIGHT);
            if guard_pos.1 == cols - 1 {
                break;
            }
            if reached.contains(&new_reached) {
                return true;
            }
            reached.insert(new_reached);
            //going down
            let (row, col) = guard_pos;
            for r in row..rows {
                if matrix[r][col] == b'#' {
                    break;
                }
                guard_pos.0 = r;
            }
            let new_reached = (guard_pos.0, guard_pos.1, Dir::DOWN);

            if guard_pos.0 == rows - 1 {
                break;
            }
            if reached.contains(&new_reached) {
                return true;
            }
            reached.insert(new_reached);

            //going left
            let (row, col) = guard_pos;
            for c in (0..col).rev() {
                if matrix[row][c] == b'#' {
                    break;
                }
                guard_pos.1 = c;
            }
            let new_reached = (guard_pos.0, guard_pos.1, Dir::LEFT);
            if guard_pos.1 == 0 {
                break;
            }
            if reached.contains(&new_reached) {
                return true;
            }
            reached.insert(new_reached);
        }
        false
    }

    let mut res = 0;

    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == b'.' {
                matrix[r][c] = b'#';
                if detect_cycle(&matrix, initial_pos) {
                    res += 1;
                }
                matrix[r][c] = b'.';
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            question06(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
            ),
            41
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            question06_part_2(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
            ),
            6
        )
    }
}
