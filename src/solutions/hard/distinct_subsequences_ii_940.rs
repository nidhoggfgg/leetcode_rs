/*
 * @lc app=leetcode.cn id=940 lang=rust
 *
 * [940] Distinct Subsequences II
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let mut end = [0_u64; 26];
        let mut total = 0_u64;
        for c in s.bytes().map(|c| (c - b'a') as usize) {
            (end[c], total) = ((total + 1) % MOD, (2 * total + 1 + MOD - end[c]) % MOD);
        }
        total as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::distinct_subseq_ii("adbae".into()), 29);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::distinct_subseq_ii("aaa".into()), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::distinct_subseq_ii("zchmliaqdgvwncfatcfivphddpzjkgyygueikthqzyeeiebczqbqhdytkoawkehkbizdmcnilcjjlpoeoqqoqpswtqdpvszfaksn".into()), 97915677);
    }
}
