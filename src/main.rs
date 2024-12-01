use std::{fs::File, io::Read};

use advent_of_code_2024_answers::questions::question01::question1;

fn main() {
    println!("question 1 answer: {}", solving_question_1());
}

fn solving_question_1() -> i64 {
    let mut input: String = String::new();
    File::open("./src/q1_input.txt")
        .expect("error opening the file")
        .read_to_string(&mut input)
        .expect("Some error converting to string");

    let mut vec1: Vec<i64> = vec![];
    let mut vec2: Vec<i64> = vec![];

    for nums in input.split('\n') {
        let nums2: Vec<_> = nums.split_whitespace().collect();
        vec1.push(nums2[0].parse().unwrap());
        vec2.push(nums2[1].parse().unwrap());
    }
    question1(vec1, vec2)
}
