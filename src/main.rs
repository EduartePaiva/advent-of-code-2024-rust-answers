#![allow(dead_code)]
use std::{fs::File, io::Read};

use advent_of_code_2024_answers::questions::{
    question01::{question01, question1_part_2},
    question02::{question02, question02_part_2},
    question03::{question03, question03_part_2},
    question04::{question04, question04_part_2},
    question05::{question05, question05_part_2},
    question06::{question06, question06_part_2},
    question07::{question07, question07_part_2},
    question08::{question08, question08_part_2},
    question09::{question09, question09_part_2},
};

fn main() {
    // println!("question 1 answer: {:?}", solving_question_1());
    // println!("question 2 answer: {:?}", solving_question_2());
    // println!("question 3 answer: {:?}", solving_question_3());
    // println!("question 4 answer: {:?}", solving_question_4());
    // println!("question 5 answer: {:?}", solving_question_5());
    // // println!("question 6 answer: {:?}", solving_question_6());
    // println!("question 7 answer: {:?}", solving_question_7());
    // println!("question 8 answer: {:?}", solving_question_8());
    println!("question 9 answer: {:?}", solving_question_9());
}

fn read_the_file(path: &str) -> String {
    let mut input: String = String::new();
    File::open(path)
        .expect("error opening the file")
        .read_to_string(&mut input)
        .expect("Some error converting to string");
    input
}

fn solving_question_1() -> (i64, i64) {
    let input = read_the_file("./src/q1_input.txt");
    let mut vec1: Vec<i64> = vec![];
    let mut vec2: Vec<i64> = vec![];

    for nums in input.lines() {
        let nums2: Vec<_> = nums.split_whitespace().collect();
        vec1.push(nums2[0].parse().unwrap());
        vec2.push(nums2[1].parse().unwrap());
    }
    (
        question01(vec1.clone(), vec2.clone()),
        question1_part_2(vec1, vec2),
    )
}

fn solving_question_2() -> (i32, i32) {
    let input_q2 = read_the_file("./src/q2_input.txt");
    let mut input: Vec<Vec<i32>> = vec![];
    for nums in input_q2.lines() {
        let nums2: Vec<i32> = nums
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        input.push(nums2);
    }
    (question02(input.clone()), question02_part_2(input))
}

fn solving_question_3() -> (i64, i64) {
    let input = read_the_file("./src/q3_input.txt");
    (question03(input.clone()), question03_part_2(input))
}

fn solving_question_4() -> (i32, i32) {
    let input_q4 = read_the_file("./src/q4_input.txt");
    let input: Vec<_> = input_q4.lines().collect();
    (question04(input.clone()), question04_part_2(input))
}
fn solving_question_5() -> (i64, i64) {
    let input = read_the_file("./src/q5_input.txt");
    (question05(&input), question05_part_2(&input))
}
fn solving_question_6() -> (i64, i64) {
    let input = read_the_file("./src/q6_input.txt");
    (question06(&input), question06_part_2(&input))
}
fn solving_question_7() -> (i64, i64) {
    let input = read_the_file("./src/q7_input.txt");
    (question07(&input), question07_part_2(&input))
}
fn solving_question_8() -> (i64, i64) {
    let input = read_the_file("./src/q8_input.txt");
    (question08(&input), question08_part_2(&input))
}
fn solving_question_9() -> (i64, i64) {
    let input = read_the_file("./src/q9_input.txt");
    (question09(&input), question09_part_2(&input))
}
