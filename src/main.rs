#![allow(non_snake_case)]

extern crate rug;
use rug::{Integer, Assign, ops::Pow};
use std::time::Instant;
use std::fs;

const ITER:usize = 1 + 28;

fn main() {
    let mut powers: [Integer; ITER] = Default::default();
    powers[0].assign(9);

    let current_power = 1;

    for i in current_power..ITER {
        let start = Instant::now();
        powers[i] = powers[i - 1].clone().pow(2);
        println!("Calculated 9^{}, in {:?}, step {}", i32::pow(2, i as u32), Instant::now() - start, i);
    }

    let to_multiply = [3, 6, 8, 12, 15, 16, 17, 18, 20, 24, 25, 26, 28];

    let mut final_result = powers[0].clone();
    for i in 0..to_multiply.len() {
        let start = Instant::now();
        print!("Calculating: {}", to_multiply[i]);

        final_result *= powers[to_multiply[i]].clone();

        println!(", done in {:?}", Instant::now() - start)
    }
    println!("Done");

    let _ = fs::write("result.txt", final_result.to_string());
}
