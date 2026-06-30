/*
 * @lc app=leetcode.cn id=860 lang=rust
 *
 * [860] Lemonade Change
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut counts = [0, 0];
        for x in bills {
            match x {
                5=>counts[0]+=1,
                10=>if counts[0]==0{return false}else{counts[0]-=1;counts[1]+=1;}
                20=>if counts[1]>0&&counts[0]>0{
                    counts[1]-=1;counts[0]-=1;
                }else if counts[0]>=3{
                    counts[0]-=3;
                }else{return false},
                _=>unreachable!()
            }
        }
        true
    }
}
// @lc code=end
