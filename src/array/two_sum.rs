/*
   Given an array of integers nums and an integer target, return indices of the two numbers such
   that they add up to target. You may assume that each input would have exactly one solution, and
   you may not use the same element twice. You can return the answer in any order.
*/

use std::collections::HashMap;

pub fn run(nums: Vec<i32>, target: i32) -> [usize; 2] {
    let mut elements: HashMap<i32, usize> = HashMap::new();

    for (index, pair_0) in nums.iter().enumerate() {
        let pair_1 = target - pair_0;

        if elements.contains_key(&pair_1) {
            return [*elements.get(&pair_1).unwrap(), index];
        } else {
            elements.insert(*pair_0, index);
        }
    }

    [0, 0]
}

pub fn leetcode_run(nums: Vec<i32>, target: i32) -> Vec<i32> {
    run(nums, target).iter().map(|e| *e as i32).collect()
}

#[test]
fn case_0() {
    assert_eq!(run(vec![2, 7, 1, 15], 9), [0, 1]);
}

#[test]
fn case_1() {
    assert_eq!(run(vec![3, 2, 4], 6), [1, 2]);
}

#[test]
fn case_2() {
    assert_eq!(run(vec![3, 3], 6), [0, 1]);
}

/*
 * 1. When the number and it's complement are same, care needs to be taken to not consider the
 *    number's presence as it's complement's presence. This is why elements are inserted in hashmap
 *    after checking for it's complement's present and not before.
 * 2. Alternative approach: Sort the array and have two pointers (one at end and another at start),
 *    and keep converging them until they intersect or desired sum is achieved.
*/
