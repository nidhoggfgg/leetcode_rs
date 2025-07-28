/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] Roman to Integer
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prefix = 0;
        let symbols = [
            ('M', 1000),
            ('D', 500),
            ('C', 100),
            ('L', 50),
            ('X', 10),
            ('V', 5),
            ('I', 1),
        ];
        for c in s.chars() {
            for (sym, v) in symbols {
                if c == sym {
                    if prefix < v {
                        result += v - 2 * prefix;
                    } else {
                        result += v;
                    }
                    prefix = v;
                }
            }
        }

        result
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let s = "IX".to_string();
        assert_eq!(Solution::roman_to_int(s), 9);
    }

    #[test]
    fn test_2() {
        let s = "III".to_string();
        assert_eq!(Solution::roman_to_int(s), 3);
    }

    #[test]
    fn test_3() {
        let s = "LVIII".to_string();
        assert_eq!(Solution::roman_to_int(s), 58)
    }

    #[test]
    fn test_4() {
        let s = "MCMXCIV".to_string();
        assert_eq!(Solution::roman_to_int(s), 1994)
    }
}
