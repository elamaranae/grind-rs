/*
    You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
    Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
    Return intervals after the insertion.
*/

use std::cmp::{max, min};

fn is_interval_overlapping(interval_1: &Vec<i32>, interval_2: &Vec<i32>) -> bool {
    let x0 = interval_1[0];
    let x1 = interval_1[1];
    let y0 = interval_2[0];
    let y1 = interval_2[1];

    (x0 >= y0 && x0 <= y1)
        || (x1 >= y0 && x1 <= y1)
        || (y0 >= x0 && y0 <= x1)
        || (y1 >= x0 && y1 <= x1)
}

pub fn run(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let start_interval_pos = intervals
        .iter()
        .position(|e| is_interval_overlapping(e, &new_interval));
    let end_interval_pos = intervals
        .iter()
        .rev()
        .position(|e| is_interval_overlapping(e, &new_interval));

    match (start_interval_pos, end_interval_pos) {
        (Some(start_index), Some(mut end_index)) => {
            end_index = intervals.len() - end_index - 1;
            intervals[start_index][0] = min(intervals[start_index][0], new_interval[0]);
            intervals[end_index][1] = max(intervals[end_index][1], new_interval[1]);
            let merge_interval = vec![intervals[start_index][0], intervals[end_index][1]];
            intervals.drain(start_index..=end_index);
            intervals.insert(start_index, merge_interval);
        }
        (None, None) => {
            let mut index = 0;

            for e in &intervals {
                if new_interval[0] > e[0] {
                    index += 1;
                }
            }

            intervals.insert(index, new_interval);
        }
        _ => {}
    }
    intervals
}

#[test]
fn case_0() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    assert_eq!(run(intervals, new_interval), vec![vec![1, 5], vec![6, 9]]);
}

#[test]
fn case_1() {
    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    assert_eq!(
        run(intervals, new_interval),
        vec![vec![1, 2], vec![3, 10], vec![12, 16]]
    );
}

#[test]
fn case_2() {
    let intervals = vec![vec![1, 5]];
    let new_interval = vec![0, 6];
    assert_eq!(run(intervals, new_interval), vec![vec![0, 6]]);
}
