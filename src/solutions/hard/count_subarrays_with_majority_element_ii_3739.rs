/*
 * @lc app=leetcode.cn id=3739 lang=rust
 *
 * [3739] Count Subarrays With Majority Element II
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let mut result = 0;
        let len = nums.len();
        let mut pre = vec![0; len*2+1];
        pre[len] = 1;
        let mut presum = 0;
        let mut cnt = len;
        for i in 0..len {
            if nums[i]==target{
                presum+=pre[cnt];
                cnt+=1;
                pre[cnt]+=1;
            } else {
                cnt-=1;
                presum-=pre[cnt];
                pre[cnt]+=1;
            }
            result+=presum;
        }
        
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1,2,2,3];
        let target = 2;
        assert_eq!(Solution::count_majority_subarrays(nums, target),5);
    }
}
