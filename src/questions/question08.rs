use std::collections::{HashMap, HashSet};

pub fn question08(input: &str) -> i64 {
    // a vec hashmap to store a vec of the positions of the same antenas
    let input: Vec<_> = input.lines().map(|v| v.as_bytes()).collect();
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();

    for r in 0..input.len() {
        for c in 0..input[0].len() {
            if input[r][c] != b'.' {
                antennas
                    .entry(input[r][c])
                    .or_insert(Vec::new())
                    .push((r as i32, c as i32));
            }
        }
    }
    // for each antenna well math the position of the antinode and then check if it's in bound
    // if that's true increase the result
    let mut locations = HashSet::new();
    for group_antenna in antennas.values() {
        for antenna in group_antenna {
            let (cur_row, cur_col) = *antenna;
            for antenna_nei in group_antenna {
                if antenna == antenna_nei {
                    continue;
                }
                let (nei_row, nei_col) = *antenna_nei;
                let (diff_r, diff_c) = (nei_row - cur_row, nei_col - cur_col);
                let (anti_r, anti_c) = (nei_row + diff_r, nei_col + diff_c);

                if anti_c > -1 && anti_c < cols && anti_r > -1 && anti_r < rows {
                    locations.insert((anti_r, anti_c));
                }
            }
        }
    }

    locations.len() as i64
}

pub fn question08_part_2(input: &str) -> i64 {
    // a vec hashmap to store a vec of the positions of the same antenas
    let input: Vec<_> = input.lines().map(|v| v.as_bytes()).collect();
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;
    let mut antennas: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();

    for r in 0..input.len() {
        for c in 0..input[0].len() {
            if input[r][c] != b'.' {
                antennas
                    .entry(input[r][c])
                    .or_insert(Vec::new())
                    .push((r as i32, c as i32));
            }
        }
    }
    // for each antenna well math the position of the antinode and then check if it's in bound
    // if that's true increase the result
    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    for group_antenna in antennas.values() {
        for antenna in group_antenna {
            let (cur_row, cur_col) = *antenna;
            for antenna_nei in group_antenna {
                if antenna == antenna_nei {
                    continue;
                }
                locations.insert(*antenna);

                let (nei_row, nei_col) = *antenna_nei;
                let (diff_r, diff_c) = (nei_row - cur_row, nei_col - cur_col);
                let (mut anti_r, mut anti_c) = (nei_row, nei_col);

                while anti_c > -1 && anti_c < cols && anti_r > -1 && anti_r < rows {
                    locations.insert((anti_r, anti_c));
                    anti_r += diff_r;
                    anti_c += diff_c;
                }
            }
        }
    }

    locations.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            question08(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
            ),
            14
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            question08_part_2(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
            ),
            34
        )
    }
}
