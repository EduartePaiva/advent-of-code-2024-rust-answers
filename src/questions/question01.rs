// advent of code question 1

pub fn question1(mut list1: Vec<i64>, mut list2: Vec<i64>) -> i64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            question1(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            11
        )
    }
}
