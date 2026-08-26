/*
 * @lc app=leetcode.cn id=2904 lang=rust
 *
 * [2904] Shortest and Lexicographically Smallest Beautiful String
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let k = k as usize;
        let mut indexs = Vec::new();

        for (i, x) in s.as_bytes().iter().enumerate() {
            if *x == b'1' {
                indexs.push(i);
            }
        }

        if indexs.len() < k {
            return "".to_string();
        }

        let mut ans: Option<&[usize]> = None;

        for chunk in indexs.windows(k as usize) {
            if let Some(c) = ans {
                let start_a = chunk[0];
                let start_b = c[0];

                let len_a = chunk.last().unwrap() - start_a;
                let len_b = c.last().unwrap() - start_b;
                if len_a != len_b {
                    if len_a < len_b {
                        ans = Some(chunk);
                    }
                    continue;
                }

                for (a, b) in chunk.iter().zip(c) {
                    if a - start_a != b - start_b {
                        if a - start_a > b - start_b {
                            ans = Some(chunk);
                        }
                        break;
                    }
                }
            } else {
                ans = Some(chunk);
            }
        }

        let ans = ans.unwrap();

        s[ans[0]..=*ans.last().unwrap()].to_string()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "1011".to_string();
        let k = 2;
        assert_eq!(
            Solution::shortest_beautiful_substring(s, k),
            "11".to_string()
        );
    }

    #[test]
    fn test_2() {
        let s = "01011101000111110".to_string();
        let k = 5;
        assert_eq!(
            Solution::shortest_beautiful_substring(s, k),
            "11111".to_string()
        );
    }
}
