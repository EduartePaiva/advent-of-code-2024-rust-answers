// advent of code question 3

fn calc_possible_mul(input: &[u8]) -> i64 {
    //negative numbers don't exists
    // numbers are 1-3 digits
    // get first number
    let mut first = 0;
    let mut input = input.iter();
    for _ in 0..4 {
        match input.next() {
            Some(v) => {
                if *v == b',' {
                    break;
                }
                if *v < b'0' || *v > b'9' {
                    return 0;
                }
                first *= 10;
                first += (v - b'0') as i64;
            }
            None => return 0,
        }
    }
    // get second number
    let mut second = 0;
    for _ in 0..4 {
        match input.next() {
            Some(v) => {
                if *v == b')' {
                    return first * second;
                }
                if *v < b'0' || *v > b'9' {
                    return 0;
                }
                second *= 10;
                second += (v - b'0') as i64;
            }
            None => return 0,
        }
    }
    0
}

pub fn question03(input: String) -> i64 {
    let input = input.as_bytes();
    let mut res = 0;
    for i in 0..input.len() - 4 {
        if input[i] == b'm' && input[i + 1] == b'u' && input[i + 2] == b'l' && input[i + 3] == b'('
        {
            let ans = calc_possible_mul(&input[i + 4..]);
            res += ans;
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
            question03(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .to_string()
            ),
            161
        );
    }
}