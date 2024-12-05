// advent of code question 1

use std::collections::HashMap;

pub fn question01(mut list1: Vec<i64>, mut list2: Vec<i64>) -> i64 {
    if list1.len() != list2.len() {
        panic!("Both lists should have same size");
    }
    list1.sort_unstable();
    list2.sort_unstable();

    let mut diff = 0;

    for i in 0..list1.len() {
        diff += list1[i].abs_diff(list2[i]) as i64;
    }

    diff
}
pub fn question1_part_2(list1: Vec<i64>, list2: Vec<i64>) -> i64 {
    let mut list_2_map = HashMap::new();
    for n in list2 {
        *list_2_map.entry(n).or_insert(0) += 1;
    }

    let mut similarity_score = 0;

    for n in list1 {
        similarity_score += n * *list_2_map.get(&n).unwrap_or(&0);
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            question01(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            11
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            question1_part_2(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            31
        )
    }
}
