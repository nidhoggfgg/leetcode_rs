/*
 * @lc app=leetcode.cn id=167 lang=rust
 *
 * [167] Two Sum II - Input Array Is Sorted
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        for i in 0..len{
            let t=target-numbers[i];
            if let Some(j) = Self::binary_search(&numbers, t, i+1, len) {
                return vec![i as i32+1,j as i32+1];
            }
        }
        unreachable!()
    }

    fn binary_search(nums:&[i32],t:i32,l:usize,r:usize)->Option<usize>{
        if l>=r{return None;}
        let m=(l+r)/2;
        if nums[m]<t{Self::binary_search(nums, t, m+1, r)}
        else if nums[m]>t{Self::binary_search(nums, t, l, m)}
        else {Some(m)}
    }
}
// @lc code=end

