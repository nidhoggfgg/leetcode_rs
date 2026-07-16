
/*
 * @lc app=leetcode.cn id=3867 lang=rust
 *
 * [3867] Sum of GCD of Formed Pairs
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn gcd_sum(mut nums: Vec<i32>) -> i64 {
        let len=nums.len();
        let mut m=nums[0];
        for x in&mut nums {
            if*x<m{*x=Self::gcd(*x, m)}
            else{m=*x}
        }
        nums.sort_unstable();
        let(mut l,mut r)=(0, len-1);
        let mut ans=0;
        while l<r{ans+=Self::gcd(nums[l],nums[r]) as i64;l+=1;r-=1;}
        ans
    }

    #[inline(always)]
    fn gcd(mut a: i32, mut b: i32) -> i32{
        while b!=0{(a,b)=(b,a%b)}a
    }
}
// @lc code=end

