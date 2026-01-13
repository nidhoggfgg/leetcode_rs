/*
 * @lc app=leetcode.cn id=3453 lang=rust
 *
 * [3453] Separate Squares I
 */

use std::i32;

use super::Solution;

// @lc code=start
impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut total_size = 0.0;
        let mut max_y = 0;
        let mut min_y = i32::MAX;
        for square in &squares {
            total_size += square[2] as f64 * square[2] as f64;
            if square[1] < min_y {
                min_y = square[1];
            }
            if square[1] + square[2] > max_y {
                max_y = square[1] + square[2];
            }
        }

        let mid_size = total_size / 2.0;

        let mut low = min_y as f64;
        let mut high = max_y as f64;
        for _ in 0..50 {
            if high - low < 0.00001 {
                break;
            }
            let mid = (high + low) / 2.0;
            let mut below_size = 0.0;
            for square in &squares {
                if square[1] as f64 > mid {
                    continue;
                }
                if (square[1] + square[2]) as f64 > mid {
                    below_size += square[2] as f64 * (mid - square[1] as f64);
                } else {
                    below_size += square[2] as f64 * square[2] as f64;
                }
            }
            if below_size < mid_size {
                low = mid;
            } else {
                high = mid;
            }
        }

        low
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn test_1() {
        let squares = vec![vec![0,0,1], vec![2,2,1]];
        let x = 1.0;
        let r = Solution::separate_squares(squares) - x;
        assert!(r < 0.00001);
    }
}
