/*
 * @lc app=leetcode.cn id=1510 lang=rust
 *
 * [1510] Stone Game IV
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let max_squre = (n as f64).sqrt() as usize;
        let first_n_squres = (1..=max_squre)
            .map(|x| (x * x) as usize)
            .collect::<Vec<_>>();
        let mut dp = (0..=n).map(|_| false).collect::<Vec<_>>();

        for x in 1..=n as usize {
            let mut i = 0;
            let mut found = false;
            while i < max_squre && x >= first_n_squres[i] {
                if !dp[x - first_n_squres[i]] {
                    found = true;
                    break;
                }
                i += 1;
            }
            dp[x] = found;
        }

        dp[n as usize]
    }
}
// @lc code=end
