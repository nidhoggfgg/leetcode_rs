/*
 * @lc app=leetcode.cn id=150 lang=rust
 *
 * [150] Evaluate Reverse Polish Notation
 */

pub struct Solution;
// @lc code=start
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums_stack = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let r = nums_stack.pop().unwrap();
                    let l = nums_stack.pop().unwrap();
                    let v = match token.as_str() {
                        "+" => l + r,
                        "-" => l - r,
                        "*" => l * r,
                        "/" => l / r,
                        _ => unreachable!()
                    };
                    nums_stack.push(v);
                }
                _ => nums_stack.push(token.parse().unwrap()),
            }
        }
        nums_stack[0]
    }
}
// @lc code=end

