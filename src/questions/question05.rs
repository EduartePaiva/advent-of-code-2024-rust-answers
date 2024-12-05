// Question 05

pub fn question05(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut after_y: Vec<Vec<usize>> = vec![vec![]; 100];
    // the first part of input is two digits wide
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        after_y[line[0..2].parse::<usize>().unwrap()].push(line[3..].parse::<usize>().unwrap());
    }

    let mut res = 0;

    for line in lines {
        let mut visit = vec![false; 100];
        let mut nums: Vec<usize> = vec![];
        let mut invalid = false;
        for num in line.split(',') {
            let parsed_num: usize = num.parse().unwrap();
            invalid = after_y[parsed_num].iter().find(|x| visit[**x]).is_some();
            if invalid {
                break;
            }
            visit[parsed_num] = true;
            nums.push(parsed_num);
        }
        if !invalid {
            res += nums[nums.len() / 2] as i64;
        }
    }

    res
}
pub fn question05_part_2(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut after_y: Vec<Vec<usize>> = vec![vec![]; 100];
    // the first part of input is two digits wide
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }
        after_y[line[0..2].parse::<usize>().unwrap()].push(line[3..].parse::<usize>().unwrap());
    }

    let mut res = 0;

    for line in lines {
        let nums: Vec<usize> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let mut nums_to_sort = nums.clone();
        nums_to_sort.sort_unstable_by(|x, y| match after_y[*y].iter().find(|n| *n == x) {
            Some(_) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Less,
        });
        if nums != nums_to_sort {
            res += nums_to_sort[nums_to_sort.len() / 2] as i64;
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
            question05(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            143
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            question05_part_2(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            123
        );
    }
}
