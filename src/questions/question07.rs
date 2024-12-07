// question 07

struct Operations {
    total: i64,
    numbers: Vec<i64>,
}

pub fn question07(input: &str) -> i64 {
    let mut operations: Vec<Operations> = Vec::new();
    for line in input.lines() {
        let mut nums = line.split_whitespace();
        let total = nums.next().unwrap();
        let total = total[..total.len() - 1].parse::<i64>().unwrap();
        let numbers: Vec<i64> = nums.map(|n| n.parse::<i64>().unwrap()).collect();
        operations.push(Operations { total, numbers });
    }

    fn dfs(prev: i64, op: &Operations, index: usize) -> bool {
        if prev > op.total {
            return false;
        }
        if index == op.numbers.len() {
            if prev == op.total {
                return true;
            } else {
                return false;
            }
        }
        dfs(prev * op.numbers[index], op, index + 1) || dfs(prev + op.numbers[index], op, index + 1)
    }
    let mut res = 0;
    for op in operations {
        if dfs(op.numbers[0], &op, 1) {
            res += op.total;
        }
    }

    res
}
pub fn question07_part_2(input: &str) -> i64 {
    let mut operations: Vec<Operations> = Vec::new();
    for line in input.lines() {
        let mut nums = line.split_whitespace();
        let total = nums.next().unwrap();
        let total = total[..total.len() - 1].parse::<i64>().unwrap();
        let numbers: Vec<i64> = nums.map(|n| n.parse::<i64>().unwrap()).collect();
        operations.push(Operations { total, numbers });
    }

    fn dfs(prev: i64, op: &Operations, index: usize) -> bool {
        if prev > op.total {
            return false;
        }
        if index == op.numbers.len() {
            if prev == op.total {
                return true;
            } else {
                return false;
            }
        }
        let current = op.numbers[index];
        let mut current_size = 0;
        let mut current_cp = current;
        while current_cp > 0 {
            current_cp /= 10;
            current_size += 1;
        }
        dfs(prev + current, op, index + 1)
            || dfs(prev * current, op, index + 1)
            || dfs((prev * 10_i64.pow(current_size)) + current, op, index + 1)
    }
    let mut res = 0;
    for op in operations {
        if dfs(op.numbers[0], &op, 1) {
            res += op.total;
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
            question07(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
            ),
            3749
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            question07_part_2(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
            ),
            11387
        );
    }
}
