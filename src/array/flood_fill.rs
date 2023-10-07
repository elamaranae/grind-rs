/*
    To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally
    to the starting pixel of the same color as the starting pixel, plus any pixels connected
    4-directionally to those pixels (also with the same color), and so on. Replace the color of all
    of the aforementioned pixels with color.
*/

pub fn run(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let m = image.len();
    let n = if m > 0 { image[0].len() } else { 0 };

    let prev = image[sr as usize][sc as usize];
    fill(&mut image, sr, sc, color, m as i32, n as i32, prev);
    image
}

fn fill(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32, m: i32, n: i32, prev: i32) {
    if sr < 0
        || sr > (m - 1)
        || sc < 0
        || sc > (n - 1)
        || image[sr as usize][sc as usize] == color
        || image[sr as usize][sc as usize] != prev
    {
        return;
    }

    image[sr as usize][sc as usize] = color;
    fill(image, sr - 1, sc, color, m, n, prev);
    fill(image, sr + 1, sc, color, m, n, prev);
    fill(image, sr, sc - 1, color, m, n, prev);
    fill(image, sr, sc + 1, color, m, n, prev);
}

#[test]
fn case_0() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let sr = 1;
    let sc = 1;
    let color = 2;
    let result = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    assert_eq!(run(image, sr, sc, color), result);
}

#[test]
fn case_1() {
    let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
    let sr = 0;
    let sc = 0;
    let color = 0;
    let result = vec![vec![0, 0, 0], vec![0, 0, 0]];
    assert_eq!(run(image, sr, sc, color), result);
}
