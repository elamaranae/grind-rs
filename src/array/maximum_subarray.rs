/* Given an integer array nums, find the subarray with the largest sum, and return its sum. */

pub fn run(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut global_sum = nums[0];
    let mut local_sum = nums[0];

    for num in &nums[1..] {
        local_sum = std::cmp::max(local_sum + num, *num);
        global_sum = std::cmp::max(local_sum, global_sum);
    }

    global_sum
}

#[test]
fn case_0() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(run(nums), 6);
}

#[test]
fn case_1() {
    let nums = vec![1];
    assert_eq!(run(nums), 1);
}

#[test]
fn case_2() {
    let nums = vec![5, 4, -1, 7, 8];
    assert_eq!(run(nums), 23);
}

#[test]
fn case_3() {
    let nums = vec![];
    assert_eq!(run(nums), 0);
}
