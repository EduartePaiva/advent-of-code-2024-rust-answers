// question 6

use std::collections::{HashSet, VecDeque};

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
    let matrix: Vec<_> = input.lines().map(|v| v.as_bytes()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
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

    let banned_pos = guard_pos;
    let mut last_tree_queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut res = 0;

    fn math_queue(
        last_tree_queue: &mut VecDeque<(usize, usize)>,
        banned_pos: (usize, usize),
    ) -> i64 {
        if last_tree_queue.len() == 4 {
            last_tree_queue.pop_front();
        }
        if last_tree_queue.len() == 3 {
            // this check is useless, always 3 dots will form the next point.
            // the trick is checking if there is no obstacles in the path to
            // the closed loop and considering the banned point

            // I have to visualize the direction of it
            // println!("{:?}", last_tree_queue);
            // let row_1 = last_tree_queue[2].0;
            // let row_2 = last_tree_queue[0].0;
            // let col_1 = last_tree_queue[2].1;
            // let col_2 = last_tree_queue[0].1;
            // if last_tree_queue[1] == (row_1, col_2) {
            //     return 1;
            // }

            // if last_tree_queue[1] == (row_2, col_1) {
            //     return 1;
            // }
        }
        0
    }

    loop {
        // going up
        let (row, col) = guard_pos;
        for r in (0..row).rev() {
            if matrix[r][col] == b'#' {
                break;
            }
            guard_pos = (r, col);
        }
        if guard_pos.0 == 0 {
            break;
        }
        last_tree_queue.push_back(guard_pos);
        res += math_queue(&mut last_tree_queue, banned_pos);
        // going right
        let (row, col) = guard_pos;
        for c in col..cols {
            if matrix[row][c] == b'#' {
                break;
            }
            guard_pos = (row, c);
        }
        if guard_pos.1 == cols - 1 {
            break;
        }
        last_tree_queue.push_back(guard_pos);
        res += math_queue(&mut last_tree_queue, banned_pos);
        //going down
        let (row, col) = guard_pos;
        for r in row..rows {
            if matrix[r][col] == b'#' {
                break;
            }
            guard_pos.0 = r;
        }
        if guard_pos.0 == rows - 1 {
            break;
        }
        last_tree_queue.push_back(guard_pos);
        res += math_queue(&mut last_tree_queue, banned_pos);
        //going left
        let (row, col) = guard_pos;
        for c in (0..col).rev() {
            if matrix[row][c] == b'#' {
                break;
            }
            guard_pos.1 = c;
        }
        if guard_pos.1 == 0 {
            break;
        }
        last_tree_queue.push_back(guard_pos);
        res += math_queue(&mut last_tree_queue, banned_pos);
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
