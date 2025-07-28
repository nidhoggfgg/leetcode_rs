/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] Container With Most Water
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_area = 0;
        loop {
            if l >= r {
                break;
            }
            let h = height[l].min(height[r]);
            let v = h * (r - l) as i32;
            max_area = max_area.max(v);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max_area
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solutions::medium::container_with_most_water_11::Solution;

    #[test]
    fn test_1() {
        let height = vec![1, 2, 3, 2, 1];
        assert_eq!(Solution::max_area(height), 4);
    }
}
