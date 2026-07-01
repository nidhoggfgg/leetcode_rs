/*
 * @lc app=leetcode.cn id=343 lang=rust
 *
 * [343] Integer Break
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n==2{return 1;}
        if n==3{return 2;}
        if n==4{return 4;}
        let a=(n/3)as u32;
        let b=n%3;
        if b==1{3_i32.pow(a-1)*4}
        else if b==0{3_i32.pow(a)}
        else{3_i32.pow(a)*2}
    }
}
// @lc code=end

