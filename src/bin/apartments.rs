use std::{
    cmp::min,
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

    if k >= apartments[apartments.len() - 1] {
        return min(applicants.len(), apartments.len()) as i32;
    }

    let mut booked: HashSet<usize> = HashSet::with_capacity(apartments.len());

    let above_lower = |i: usize, appl: i32| apartments[i] >= appl - k;
    let below_upper = |i: usize, appl: i32| apartments[i] <= appl + k;

    for appl in &applicants {
        let mut low = 0;
        let mut high = apartments.len() - 1;
        let mut size = high - low;
        while high - low > 1 {
            let mid = low + size / 2;

            if above_lower(mid, *appl) && !booked.contains(&mid) {
                high = mid;
            } else {
                low = mid;
            }
            size = high - low;
        }

        if above_lower(high, *appl) && below_upper(high, *appl) {
            booked.insert(high);
        } else if apartments.len() == 2 && above_lower(low, *appl) && below_upper(low, *appl) {
            // HACK if array len less then 3 - binary search dont start
            booked.insert(low);
        }
    }

    booked.len() as i32
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
