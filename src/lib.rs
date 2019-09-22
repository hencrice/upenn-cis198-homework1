#![allow(dead_code)]

use std::collections::HashSet;

// Uncomment these to have Rust compile the other files as well.
mod lib2;
// mod lib3;

// Part 1. Implementing Functions. Taken from Fall 2016's Rust class.
// Write unit tests for you functions.

// Problem 1
// Implement the sum function on slices. Do not use the predefined sum function.
fn sum(slice: &[i32]) -> i32 {
    // but but but......I want to be functional lol
    slice.iter().sum()
}

// Problem 2.
// Make unique. Create a new vector which contains each item in the vector
// only once! Much like a set would.
// Please implement this using a for loop.
fn unique(vs: &[i32]) -> Vec<i32> {
    // clippy warns for the signature above:
    // warning: writing `&Vec<_>` instead of `&[_]` involves one more reference
    // and cannot be used with non-Vec-based slices
    let mut result: Vec<i32> = Vec::new();
    let mut set = HashSet::new();

    for i in vs {
        if set.contains(i) {
            continue;
        }
        // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hash-maps-and-ownership
        // the value is copied instead of moved
        set.insert(*i);
        result.push(*i);
    }
    result
}

// Problem 3.
// return a new vector containing only elements that satisfy `pred`.
fn filter(vs: &[i32], pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut res = vec![];
    for i in vs {
        if pred(*i) {
            res.push(*i);
        }
    }
    res
}

// Problem 4
// Given starting fibonacci numbers n1 and n2, compute a vector
// where v[i] is the ith fibonacci number.
fn fibonacci(n1: i32, n2: i32, how_many: usize) -> Vec<i32> {
    match how_many {
        0 => vec![n1],
        1 => vec![n1, n2],
        _ => {
            let mut res = vec![n1, n2];
            for i in 2..=how_many {
                res.push(res[i - 1] + res[i - 2]);
            }
            res
        }
    }
}

// Problem 5
// Create a function which concats 2 strs and returns a String.
// You may use any standard library function you wish.
fn str_concat(s1: &str, s2: &str) -> String {
    vec![s1, s2].join("")
}

// Problem 7
// Convert a Vec<String> into a Vec<u64>. Assume all strings
// are correct numbers! We will do error handling later. Use
// `.expect("ignoring error")` to ignore Result from parse()
// See https://doc.rust-lang.org/std/primitive.str.html#method.parse
// Use turbo fish! Do not use type inference for parse()
fn concat_all(v: Vec<String>) -> Vec<u64> {
    v.iter()
        .map(|s| s.parse::<u64>().expect("ignoring error"))
        .collect()

    // let mut res = Vec::with_capacity(v.len());
    // for s in v {
    //     res.push(s.parse::<u64>().expect("ignoring error"))
    // }
    // res
}

// Implement concat_all using map, parse (with turbo fish), and collect()
// Check out how the lecture does something similar:
// https://github.com/upenn-cis198/lecture2/blob/f54753527c1dabbd5e55c2f48a19745768769beb/src/lib.rs#L362
fn concat_all_with_map(v: Vec<String>) -> Vec<u64> {
    v.iter()
        .map(|s| s.parse::<u64>().expect("ignoring error"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_small() {
        let array = [1, 2, 3, 4, 5];
        assert_eq!(sum(&array), 15);
    }

    #[test]
    fn test_sum_one_element() {
        let array = [1];
        assert_eq!(sum(&array), 1);
    }

    #[test]
    fn test_unique_small() {
        let vs = vec![1, 2, 2, 3, 4, 1];
        assert_eq!(unique(&vs), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_unique_no_dup() {
        let vs = vec![1, 2, 3, 4];
        assert_eq!(unique(&vs), vec![1, 2, 3, 4]);
    }

    #[test]
    fn filter_tests() {
        assert_eq!(
            filter(&vec![1, 2, 3, 4, 5, 6], &|n| n % 2 == 0),
            vec![2, 4, 6]
        );
    }

    #[test]
    fn test_filter_bigger_than_5() {
        let vs = vec![2, 4, 5, 6];
        assert_eq!(filter(&vs, &|i| i > 5), vec![6]);
    }

    #[test]
    fn test_filter_smaller_than_5() {
        let vs = vec![2, 4, 5, 6];
        assert_eq!(filter(&vs, &|i| i < 5), vec![2, 4]);
    }

    #[test]
    fn test_fibonacci_i_0() {
        assert_eq!(fibonacci(2, 1, 0), vec![2]);
    }

    #[test]
    fn test_fibonacci_i_1() {
        assert_eq!(fibonacci(2, 1, 1), vec![2, 1]);
    }

    #[test]
    fn test_fibonacci_i_5() {
        assert_eq!(fibonacci(2, 1, 5), vec![2, 1, 3, 4, 7, 11]);
    }

    #[test]
    fn test_str_concat() {
        assert_eq!(str_concat("1", "2"), "12");
    }

    #[test]
    fn test_concat_all_ok() {
        assert_eq!(
            concat_all(
                vec!["1", "2", "4", "2"]
                    .iter()
                    .map(|&s| String::from(s))
                    .collect()
            ),
            vec![1, 2, 4, 2]
        );
    }

    #[test]
    #[should_panic(expected = "ParseIntError")]
    fn test_concat_all_non_number() {
        assert_eq!(
            concat_all(
                vec!["1", "2", "chicken wings", "edna"]
                    .iter()
                    .map(|&s| String::from(s))
                    .collect()
            ),
            vec![1]
        );
    }

    #[test]
    fn test_concat_all_with_map() {
        assert_eq!(
            concat_all_with_map(
                vec!["1", "100", "1000"]
                    .iter()
                    .map(|&s| String::from(s))
                    .collect()
            ),
            vec![1, 100, 1000]
        )
    }
}
