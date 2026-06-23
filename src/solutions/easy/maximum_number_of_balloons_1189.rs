/*
 * @lc app=leetcode.cn id=1189 lang=rust
 *
 * [1189] Maximum Number of Balloons
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counts = [0; 5];
        text.chars().for_each(|x| match x {
            'a'=>counts[0]+=1,
            'b'=>counts[1]+=1,
            'l'=>counts[2]+=1,
            'o'=>counts[3]+=1,
            'n'=>counts[4]+=1,
            _ => {}
        });
        counts[2]/=2;
        counts[3]/=2;
        *counts.iter().min().unwrap()
    }
}
// @lc code=end

