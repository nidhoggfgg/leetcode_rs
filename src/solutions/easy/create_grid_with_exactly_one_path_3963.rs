/*
 * @lc app=leetcode.cn id=3963 lang=rust
 *
 * [3963] Create Grid With Exactly One Path
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let (m, n) = (n as usize, m as usize);
        if m == 1 {
            return vec![".".to_string(); n];
        }
        if n == 1 {
            return vec![".".repeat(m)];
        }
        let mut result = Vec::with_capacity(n);
        let mut k = 0;
        for _ in 0..n - 1 {
            let mut s = "#".repeat(k);
            s.push('.');
            if k == m - 1 {
                result.push(s);
                continue;
            }
            s.push('.');
            for _ in 0..(m - k - 2) {
                s.push('#');
            }
            result.push(s);
            k += 1;
        }
        let mut s = "#".repeat(k);
        s.push_str(&".".repeat(m - k));
        result.push(s);
        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::create_grid(2, 3),
            vec!["..#".to_string(), "#..".to_string()]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::create_grid(3, 2),
            vec!["..".to_string(), "#.".to_string(), "#.".to_string()]
        );
    }
}
