/*
 * @lc app=leetcode.cn id=1665 lang=rust
 *
 * [1665] Minimum Initial Energy to Finish Tasks
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        tasks.iter_mut().for_each(|x| x.push(x[1] - x[0]));
        tasks.sort_unstable_by_key(|k| k[2]);
        let mut now = 0;
        let mut start = 0;
        for x in tasks.into_iter().rev() {
            if x[1] > now {
                start += x[1] - now;
                now += x[1] - now;
            } else if x[0] > now {
                start += x[0] - now;
                now += x[0] - now;
            }
            
            now -= x[0]
        }
        
        start
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let input = vec![[1,3],[2,4],[10,11],[10,12],[8,9]];
        let input = input.iter().map(|x| x.to_vec()).collect();
        assert_eq!(Solution::minimum_effort(input), 32);
    }
}
