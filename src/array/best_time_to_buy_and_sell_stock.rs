/* 
   You are given an array prices where prices[i] is the price of a given stock on the ith day.

   You want to maximize your profit by choosing a single day to buy one stock and choosing a
   different day in the future to sell that stock.

   Return the maximum profit you can achieve from this transaction. If you cannot achieve any
   profit, return 0. 
*/

use std::cmp::{min, max};

#[must_use]
pub fn run(prices: &[i32]) -> i32 {
    let mut max_profit = 0;
    let mut current_min = prices[0];

    for price in &prices[1..] {
        max_profit = max(max_profit, price - current_min);
        current_min = min(current_min, *price);
    }

    max_profit
}

#[test]
fn case_0() {
    let prices = vec![7,1,5,3,6,4];
    assert_eq!(run(&prices), 5);
}

#[test]
fn case_1() {
    let prices = vec![7,6,4,3,1];
    assert_eq!(run(&prices), 0);
}


/*
   The stock can be sold (if sold) only in any of [1..n-1]th day. And on any selling day, i, the
   lowest value it could've been bought for is minimum of [0..i-1]. The intuition for this DP
   problem is just DP problem of finding minimum or maximum. (minimum of new array with one extra
   element is minimum of existing array and new element).
*/
