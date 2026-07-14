/*
 * @lc app=leetcode.cn id=1291 lang=rust
 *
 * [1291] Sequential Digits
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans=Vec::new();
        for len in 2..=9{
            let mut num=0;
            for digit in 1..=len{num=num*10+digit;}
            let mut step =0;
            for _ in 0..len{step=step*10+1;}
            for _ in 0..(10-len){
                if num>high{break;}
                if num>=low{ans.push(num);}
                num+=step;
            }
        }
        ans
    }
}
// @lc code=end

