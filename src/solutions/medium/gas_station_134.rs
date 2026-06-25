
/*
 * @lc app=leetcode.cn id=134 lang=rust
 *
 * [134] Gas Station
 */
pub struct Solution;

// @lc code=start
use std::iter::zip;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let addon = zip(gas, cost).map(|(a, b)| a-b).collect::<Vec<i32>>();
        if addon.iter().sum::<i32>()<0{return -1;}
        let mut rest = 0;
        let mut min = 0;
        for x in &addon {
            rest+=x;if rest<0{min=min.min(rest);}
        }
        if min>=0{return 0;}
        rest=min;
        for (i, x) in addon.into_iter().enumerate().rev() {
            rest+=x;if rest>=0{return i as i32;}
        }
        -1
    }
}
// @lc code=end

