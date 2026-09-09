/*
 * @lc app=leetcode.cn id=3871 lang=rust
 *
 * [3871] Count Commas in Range II
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut ans = 0;
        let mut count = 0;

        let mut x = n;
        while x / 1000 > 0 {
            x /= 1000;
            count += 1;
        }

        ans += count * (n - 1000_i64.pow(count as u32) + 1);
        count -= 1;
        while count > 0 {
            ans += count * (1000_i64.pow(count as u32 + 1) - 1000_i64.pow(count as u32));
            count -= 1;
        }

        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_commas(1_409_752_114), 3228255345);
    }
}
