// question 04

// up, down, left, right, (up,left)//, (up, right)//, (down, left), (down,right)
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];
const MAS: [u8; 3] = [b'M', b'A', b'S'];
fn is_xmas(input: &Vec<Vec<u8>>, r: i32, c: i32) -> i32 {
    // search for mas
    let mut res = 0;
    for (pos_r, pos_c) in DIRECTIONS {
        let (mut new_r, mut new_c) = (r, c);
        let mut is_good = true;
        for i in 0..3 {
            new_r += pos_r;
            new_c += pos_c;

            if new_r < 0
                || new_c < 0
                || new_r == input.len() as i32
                || new_c == input[0].len() as i32
                || input[new_r as usize][new_c as usize] != MAS[i]
            {
                is_good = false;
                break;
            }
        }
        if is_good {
            res += 1;
        }
    }

    res
}

pub fn question04(input: Vec<&str>) -> i32 {
    let input: Vec<Vec<u8>> = input.into_iter().map(|x| x.bytes().collect()).collect();
    let rows = input.len();
    let cols = input[0].len();
    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] == b'X' {
                res += is_xmas(&input, r as i32, c as i32);
            }
        }
    }
    res
}

pub fn question04_part_2(input: Vec<&str>) -> i32 {
    let input: Vec<_> = input.into_iter().map(|x| x.as_bytes()).collect();

    let rows = input.len();
    let cols = input[0].len();
    let mut res = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if input[r][c] == b'A' {
                let ms_1 = input[r - 1][c - 1];
                let ms_2 = input[r + 1][c + 1];
                let ms_3 = input[r - 1][c + 1];
                let ms_4 = input[r + 1][c - 1];
                let ms = [ms_1, ms_2, ms_3, ms_4];
                let mut is_ms = true;
                for i in 0..ms.len() {
                    if !(ms[i] == b'M' || ms[i] == b'S') {
                        is_ms = false;
                        break;
                    }
                }
                if is_ms && ms_1 != ms_2 && ms_3 != ms_4 {
                    res += 1;
                }
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
            question04(vec![
                "MMMSXXMASM",
                "MSAMXMSMSA",
                "AMXSXMAAMM",
                "MSAMASMSMX",
                "XMASAMXAMM",
                "XXAMMXXAMA",
                "SMSMSASXSS",
                "SAXAMASAAA",
                "MAMMMXMMMM",
                "MXMXAXMASX"
            ]),
            18
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            question04_part_2(vec![
                ".M.S......",
                "..A..MSMS.",
                ".M.S.MAA..",
                "..A.ASMSM.",
                ".M.S.M....",
                "..........",
                "S.S.S.S.S.",
                ".A.A.A.A..",
                "M.M.M.M.M.",
                ".........."
            ]),
            9
        )
    }
}
