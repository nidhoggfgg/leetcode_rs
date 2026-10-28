/*
 * @lc app=leetcode.cn id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let offset = 10_000;
        let mut counts = [0; 20_001];
        for x in nums {
            counts[(x + offset) as usize] += 1;
        }

        let mut k = k;
        for i in (0..20_001).rev() {
            if counts[i] >= k {
                return i as i32 - offset;
            }
            k -= counts[i];
        }
        unreachable!()
    }
}
// @lc code=end
