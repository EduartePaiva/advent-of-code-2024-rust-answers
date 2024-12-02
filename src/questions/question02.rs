// advent of code question 2

fn check_report(report: Vec<i32>) -> bool {
    let increasing = report[0] < report[1];
    let mut is_safe = true;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if increasing && (diff <= 0 || diff > 3) {
            is_safe = false;
            break;
        }
        if !increasing && (diff >= 0 || diff < -3) {
            is_safe = false;
            break;
        }
    }
    is_safe
}

pub fn question02(input: Vec<Vec<i32>>) -> i32 {
    // The levels are either all increasing or all decreasing.
    //Any two adjacent levels differ by at least one and at most three.

    let mut res = 0;
    for report in input {
        if check_report(report) {
            res += 1;
        }
    }

    res
}
pub fn question02_1_safe(input: Vec<Vec<i32>>) -> i32 {
    // The levels are either all increasing or all decreasing.
    //Any two adjacent levels differ by at least one and at most three.
    // now it can tolerate a single bad level

    let mut res = 0;
    for report in input {
        // brute force
        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if check_report(new_report) {
                res += 1;
                break;
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
            question02(vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]),
            2
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            question02_1_safe(vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]),
            4
        )
    }
}
