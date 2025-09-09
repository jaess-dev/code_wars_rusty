// https://www.codewars.com/kata/52503c77e5b972f21600000e

use std::iter;

fn largest_difference(data: &[i32]) -> usize {
    for dist in (0..=data.len()).rev() {
        for window in data.windows(dist + 1) {
            if window[0] <= window[dist] {
                return dist;
            }
        }
    }

    0
}

fn largest_difference_functional(data: &[i32]) -> usize {
    (0..data.len())
        .rev()
        .flat_map(|dist| iter::repeat(dist).zip(data.windows(dist + 1)))
        .find_map(|(dist, win)| {
            if win[0] <= win[dist] {
                Some(dist)
            } else {
                None
            }
        })
        .unwrap_or(0)
}

#[inline]
fn dist(a: usize, b: usize) -> usize {
    b - a
}

fn works_but_not_elegant(data: &[i32]) -> usize {
    let mut largest_diff = 0;
    for i in 0..data.len() {
        if dist(i, data.len()) <= largest_diff {
            break;
        }

        for j in (i + 1..data.len()).rev() {
            if dist(i, j) <= largest_diff {
                break;
            }

            if data[i] <= data[j] {
                largest_diff = dist(i, j);
                break;
            }
        }
    }

    largest_diff
}

#[cfg(test)]
mod tests {
    use crate::kyu5::largest_difference_in_increasing_indexes::{
        largest_difference_functional, works_but_not_elegant,
    };

    use super::largest_difference;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(largest_diff_fun: fn(&[i32]) -> usize, data: &[i32], expected: usize) {
        assert_eq!(
            largest_diff_fun(data),
            expected,
            "{ERR_MSG} with data = {data:?}"
        )
    }

    fn tests(largest_diff_fun: fn(&[i32]) -> usize) {
        dotest(largest_diff_fun, &[9, 4, 1, 10, 3, 4, 0, -1, -2], 4);
        dotest(largest_diff_fun, &[3, 2, 1], 0);
        dotest(largest_diff_fun, &[1, 2, 3], 2);
        dotest(largest_diff_fun, &[9, 4, 1, 2, 3, 0, -1, -2], 2);
        dotest(largest_diff_fun, &[9, 4, 1, 2, 3, 4, 0, -1, -2], 4);
        dotest(
            largest_diff_fun,
            &[78, 88, 64, 94, 17, 91, 57, 69, 38, 62, 13, 17, 35, 15, 20],
            10,
        );
        dotest(largest_diff_fun, &[4, 3, 3, 1, 5, 2], 4);
    }

    #[test]
    fn largest_difference_tests() {
        tests(largest_difference)
    }

    #[test]
    fn largest_difference_functional_tests() {
        tests(largest_difference_functional)
    }

    #[test]
    fn works_but_not_elegant_tests() {
        tests(works_but_not_elegant)
    }
}
