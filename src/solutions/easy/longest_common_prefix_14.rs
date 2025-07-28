/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut css: Vec<_> = strs.iter().map(|x| x.chars()).collect();
        let mut num = 0;
        loop {
            let now = if let Some(x) = css[0].nth(0) {
                x
            } else {
                break;
            };

            let mut end = false;
            for cs in css.iter_mut().skip(1) {
                if let Some(x) = cs.nth(0) {
                    if x != now {
                        end = true;
                        break;
                    }
                } else {
                    end = true;
                    break;
                }
            }
            if end {
                break;
            }
            num += 1;
        }
        strs[0].chars().take(num).collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let strs = vec!["a", "ab"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::longest_common_prefix(strs), "a");
    }

    #[test]
    fn test_2() {
        let strs = vec!["a", "b", "c"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::longest_common_prefix(strs), "");
    }

    #[test]
    fn test_3() {
        let strs = vec!["flower", "flow", "flight"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(strs), "fl");
    }
}
