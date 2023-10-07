/*
 Given an array of integers nums which is sorted in ascending order, and an integer target, write a
 function to search target in nums. If target exists, then return its index. Otherwise, return -1.
*/

use std::cmp::Ordering;

pub fn run(nums: Vec<i32>, target: i32) -> i32 {
    let length: i32 = nums.len() as i32;
    binary_search(&nums, 0, length - 1, target) 
}

fn binary_search(nums: &Vec<i32>, start: i32, end: i32, target: i32) -> i32 {
    if start > end { return -1 };

    let mid = (start + end)/2;

    match nums[mid as usize].cmp(&target) {
        Ordering::Equal => mid,
        Ordering::Greater => binary_search(nums, start, mid-1, target),
        Ordering::Less => binary_search(nums, mid+1, end, target)
    }
}

#[test]
fn case_0() {
    let nums = Vec::from([1,2,3,4]);
    assert_eq!(run(nums, 3), 2);
}

#[test]
fn case_1() {
    let nums = Vec::from([1,2,3,4]);
    assert_eq!(run(nums, 9), -1);
}

#[test]
fn case_2() {
    let nums = Vec::from([]);
    assert_eq!(run(nums, 9), -1);
}
