/*
 * @lc app=leetcode.cn id=3483 lang=rust
 *
 * [3483] Unique 3-Digit Even Numbers
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut counts = [0; 10];
        for x in digits {
            counts[x as usize] += 1;
        }

        let mut ans = 0;

        for first in 1..=9 {
            if counts[first] == 0 {
                continue;
            }
            counts[first] -= 1;
            for second in 0..=9 {
                if counts[second] == 0 {
                    continue;
                }
                counts[second] -= 1;
                for last in (0..=8).step_by(2) {
                    if counts[last] != 0 {
                        ans += 1;
                    }
                }
                counts[second] += 1;
            }
            counts[first] += 1;
        }

        ans
    }
}
// @lc code=end
