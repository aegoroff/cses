use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());
    let (result, _) = solution(reader);
    println!("{result}");
}

pub fn solution<B: BufRead>(mut r: B) -> (String, usize) {
    let lines = read_lines(&mut r, 1);
    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    let n = parts[0].parse::<usize>().unwrap_or(0);
    let x = parts[1].parse::<i32>().unwrap_or(0);

    let lines = read_lines(&mut r, 1);
    let parts: Vec<&str> = lines[0].split_whitespace().collect();

    let nums: Vec<i32> = parts
        .iter()
        .take(n)
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();

    let mut count = 0;

    let mut r = 0;
    let mut sum = 0;
    for l in 0..nums.len() {
        while r < nums.len() && sum < x {
            if r > l {
                sum += nums[r];
            } else {
                sum += nums[l];
            }

            r += 1;
        }
        if sum == x {
            count += 1;
        }
        sum -= nums[l];
    }

    (count.to_string(), 1)
}

fn read_lines<B: BufRead>(src: &mut B, n: i32) -> Vec<String> {
    let mut res = vec![];

    for _ in 0..n {
        let mut input = String::new();

        match src.read_line(&mut input) {
            Ok(_n) => {
                res.push(input);
            }

            Err(error) => println!("error: {}", error),
        }
    }

    res
}
#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::{
        ffi::OsStr,
        fs::{self, File},
        path::Path,
    };

    use cses::run_test_suite;

    use super::*;

    #[test]
    fn test_suite() {
        run_test_suite!("/home/egr/Downloads/sumarray_sums_tests", solution, true);
    }
}
