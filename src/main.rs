use std::{fs::File, io::Read};

use advent_of_code_2024_answers::questions::{
    question01::{question1, question1_part_2},
    question02::{question02, question02_1_safe},
};

fn main() {
    println!("question 1 answer: {:?}", solving_question_1());
    println!("question 2 answer: {:?}", solving_question_2());
}

fn solving_question_1() -> (i64, i64) {
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
    (
        question1(vec1.clone(), vec2.clone()),
        question1_part_2(vec1, vec2),
    )
}

fn solving_question_2() -> (i32, i32) {
    let mut input_q2: String = String::new();
    File::open("./src/q2_input.txt")
        .expect("error opening the file")
        .read_to_string(&mut input_q2)
        .expect("Some error converting to string");

    let mut input: Vec<Vec<i32>> = vec![];
    for nums in input_q2.split('\n') {
        let nums2: Vec<i32> = nums
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        input.push(nums2);
    }
    (question02(input.clone()), question02_1_safe(input))
}
