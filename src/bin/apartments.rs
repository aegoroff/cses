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
    let m = parts[1].parse::<usize>().unwrap_or(0);
    let k = parts[2].parse::<i32>().unwrap_or(0);

    let lines = read_lines(&mut r, 2);
    let applicants_parts: Vec<&str> = lines[0].split_whitespace().collect();
    let apartments_parts: Vec<&str> = lines[1].split_whitespace().collect();

    let mut applicants: Vec<i32> = applicants_parts
        .iter()
        .take(n)
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
    let mut apartments: Vec<i32> = apartments_parts
        .iter()
        .take(m)
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();

    applicants.sort();
    apartments.sort();

    let mut i = 0;
    let mut j = 0;
    let mut result = 0;

    while i < n && j < m {
        if apartments[j] >= applicants[i] - k && apartments[j] <= applicants[i] + k {
            j += 1;
            result += 1;
        } else if apartments[j] < applicants[i] + k {
            j += 1;
            i -= 1;
        }
        i += 1;
    }
    result
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
        run_test_suite!("/home/egr/Downloads/apartments_tests", solution);
    }
}
