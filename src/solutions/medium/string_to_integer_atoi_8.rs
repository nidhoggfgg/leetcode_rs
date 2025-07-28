/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

pub struct Solution;

// TODO: ugly, maybe i should take a look of parse()
// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = Vec::new();
        let mut is_neg = false;
        let mut is_start = true;
        let mut prefix_zero = true;
        for c in s.chars() {
            if is_start {
                match c {
                    '-' => {
                        is_start = false;
                        is_neg = true;
                        continue;
                    }
                    '+' => {
                        is_start = false;
                        is_neg = false;
                        continue;
                    }
                    ' ' => continue,
                    '0'..='9' => is_start = false,
                    _ => break,
                }
            }

            if '0' <= c && c <= '9' {
                if c == '0' && prefix_zero {
                    continue;
                }
                prefix_zero = false;
                result.push(c);
            } else {
                break;
            }
        }
        if result.len() > 10 {
            // > 2 ^ 32
            if is_neg {
                return i32::MIN;
            } else {
                return i32::MAX;
            }
        } else {
            let s: String = result.iter().collect();
            let v: i64 = s.parse().unwrap_or(0);
            if is_neg {
                if -v < i32::MIN as i64 {
                    return i32::MIN;
                } else {
                    return -v as i32;
                }
            } else {
                if v > i32::MAX as i64 {
                    return i32::MAX;
                } else {
                    return v as i32;
                }
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use crate::solutions::medium::string_to_integer_atoi_8::Solution;

    #[test]
    fn test_1() {
        let s = "1".to_string();
        assert_eq!(Solution::my_atoi(s), 1);
    }

    #[test]
    fn test_2() {
        let s = "-1".to_string();
        assert_eq!(Solution::my_atoi(s), -1);
    }

    #[test]
    fn test_3() {
        let s = "a1".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
    }

    #[test]
    fn test_4() {
        let s = "-1a".to_string();
        assert_eq!(Solution::my_atoi(s), -1);
    }

    #[test]
    fn test_5() {
        let s = "11111111111111111111111111111111111111111111111111111111111111".to_string();
        assert_eq!(Solution::my_atoi(s), i32::MAX);
    }

    #[test]
    fn test_6() {
        let s = "-11111111111111111111111111111111111111111111111111111111111111".to_string();
        assert_eq!(Solution::my_atoi(s), i32::MIN);
    }

    #[test]
    fn test_7() {
        let s = "   -042".to_string();
        assert_eq!(Solution::my_atoi(s), -42);
    }

    #[test]
    fn test_8() {
        let s = "0-1".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
    }

    #[test]
    fn test_9() {
        let s = "21474836460".to_string();
        assert_eq!(Solution::my_atoi(s), i32::MAX);
    }
}
