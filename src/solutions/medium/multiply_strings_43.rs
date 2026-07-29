
/*
 * @lc app=leetcode.cn id=43 lang=rust
 *
 * [43] Multiply Strings
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1=="0"||num2=="0"{return "0".to_string();}
        let zero='0' as u32;
        let mut ans= vec![0; num1.len()+num2.len()];
        for (i, a) in num1.chars().rev().enumerate() {
            for (j, b) in num2.chars().rev().enumerate() {
                let now=i+j;
                let s = (a as u32-zero)*(b as u32-zero)+ans[now];
                ans[now]=s%10;
                ans[now+1]+=s/10;
            }
        }
        let mut z=true;
        let mut result = String::new();
        for x in ans.into_iter().rev() {
            if z&&x==0{continue;}
            if z&&x!=0{z=false;}
            result.push((x+zero) as u8 as char);
        }
        result
    }
}
// @lc code=end
