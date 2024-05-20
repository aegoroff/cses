use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

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

    let mut seated: HashSet<usize> = HashSet::with_capacity(nums.len());
    let mut count = 0;
    for i in 0..nums.len() {
        if seated.contains(&i) {
            continue;
        }
        let current = nums[i];

        let mut low = if i + 1 < nums.len() - 1 {
            i + 1
        } else {
            nums.len() - 1
        };
        let mut high = nums.len() - 1;
        let mut size = high - low;
        while high - low > 1 {
            let mid = low + size / 2;

            if current + nums[mid] < x && !seated.contains(&mid) {
                low = mid;
            } else {
                high = mid;
            }
            size = high - low;
        }

        let other = if seated.contains(&high) || current + nums[high] > x {
            high - 1
        } else {
            high
        };

        if current + nums[other] <= x {
            seated.insert(i);
            seated.insert(other);
        }
        count += 1;
    }

    if count == 0 {
        1
    } else {
        count
    }
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
