use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());
    let result = solution(reader);
    println!("{result}");
}

pub fn solution<B: BufRead>(mut r: B) -> i32 {
    let lines = read_lines(&mut r, 1);
    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    let n = parts[0].parse::<usize>().unwrap_or(0);
    let x = parts[1].parse::<i32>().unwrap_or(0);

    let lines = read_lines(&mut r, 1);
    let parts: Vec<&str> = lines[0].split_whitespace().collect();

    let mut nums: Vec<i32> = parts
        .iter()
        .take(n)
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();

    nums.sort();

    let mut count = 0;
    let mut r = nums.len() - 1;
    for l in 0..nums.len() {
        while l < r && nums[l] + nums[r] > x {
            count += 1;
            r -= 1;
        }
        if r < l {
            break;
        }
        count += 1;
        r = r.saturating_sub(1);
    }

    count
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
        run_test_suite!("/home/egr/Downloads/ferris_tests", solution);
    }
}
