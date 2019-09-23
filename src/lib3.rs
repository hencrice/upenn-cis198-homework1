// Part 3.
// Lifetimes and move semantics.

// Problem 1.
// What went wrong? Copy strings properly.
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}

// Problem 2.
// Question 2: How come it works fine here?
#[test]
fn copy_int_test() {
    let i1 = 1;
    // integer implements the Copy trait
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Problem 3.
// These two don't work either. Fix by changing the type of "string" in the function
// copy_me ONLY, and by adjusting the parameter to "copy_me" where it's called.
#[test]
fn eat_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(&str1));
}

#[test]
fn eat_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1);
    assert_eq!(str1, str2);
}

fn copy_me(string: &str) -> String {
    string.to_string()
}

// Problem 4.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
// fn new_ref_string() -> &String {
//     unimplemented!();
// }
// A: Don't think this possible because the string allocated in the function body
// will be deallocated once the function returns, so it's impossible to return a
// reference to the deallocated string.

// Problem 5.
// Can you implement this function?
// Add a lifetime specifier to the return type if you think it will help.
// If not, why not.
// fn new_ref_str() -> & str {
//     unimplemented!();
// }
// A: Same reason as above I think.

// Problem 6.
// Now we know how to implement this type of function. Implement it and write a test
// pick_longest_tests2() which passes all tests.
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

#[test]
fn pick_longest_tests() {
    assert_eq!(pick_longest2("cat", "dog"), "cat");
}

fn find_lesser_values<'a, 'b>(input: &'a [&'a str], comp: &'b str) -> Vec<&'a str> {
    input
        .iter()
        .filter(|&&s| s < comp)
        .copied()
        .collect::<Vec<&str>>()
}

// Problem 7.
// Write a function with a type signature which type checks the following test:
// and passes the test.
// This function compares it's second argument againsts all elements in it's first
// argument, returning those that are less than (<).
#[test]
fn test_find_lesser_values() {
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], "zzzzzzzz"),
        vec!["foo", "bar", "foobar"]
    );
    assert_eq!(
        find_lesser_values(&vec!["foo", "bar", "foobar"], "bars"),
        vec!["bar"]
    );
    // Add more tests.
}

// Problem 8
// Move semantics present intersting API design choices not found in other languages.
// HashMap is an example of such a API.
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

// Specifically, the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

use std::collections::HashMap;

// Implement the following function which converts pairs from a vector,
// into key-value pairs in a hashmap.

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    v.iter()
        .map(|s| (s.0, s.1.clone()))
        .collect::<HashMap<_, _>>()
}

#[test]
fn test_vector_to_hashmap() {
    use std::iter::FromIterator;
    let expected = vec![(1, "a"), (2, "b")]
        .into_iter()
        .map(|(k, v)| (k, v.to_string()));
    assert_eq!(
        vector_to_hashmap(&[(1, "a".to_string()), (2, "b".to_string())]),
        HashMap::from_iter(expected)
    )
}

// Rust prevents us from deleting entries while iterating... Rewrite
// this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    // for k in h.keys() {
    //     if *k < 0 {
    //         h.remove(k);
    //     }
    // }
    h.retain(|&k, _| k >= 0)
}

#[test]
fn test_delete_negative_keys() {
    use std::iter::FromIterator;
    let expected = vec![(1, 2), (-1, 3)];
    let h = &mut HashMap::from_iter(expected.into_iter());
    delete_negative_keys(h);
    assert_eq!(*h, HashMap::from_iter(vec![(1, 2)].into_iter()));
}

// For all entries in `add`: (k, v)
// If `k` exists in `merged`, append `v` to the value of `merged[k]`.
// If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.

// Use the Entry API:
// https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
// Use `or_insert` and `and_modify`.
fn merge_maps(merged: &mut HashMap<String, String>, add: HashMap<String, String>) {
    for (k, v) in add {
        merged.entry(k).and_modify(|e| e.push_str(&v)).or_insert(v);
    }
}

#[test]
fn test_merge_maps() {
    use std::iter::FromIterator;
    let input = &mut HashMap::from_iter(
        vec![("a", "1"), ("b", "2")]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string())),
    );
    let to_add = HashMap::from_iter(
        vec![("a", "2"), ("c", "3")]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string())),
    );

    let expected = HashMap::from_iter(
        vec![("a", "12"), ("b", "2"), ("c", "3")]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string())),
    );

    merge_maps(input, to_add);
    assert_eq!(*input, expected);
}
