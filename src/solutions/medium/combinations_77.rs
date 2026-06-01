/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] Combinations
 */

pub struct Solution;

// @lc code=start
impl Solution {
    // pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    //     let mut result = Vec::with_capacity((n*k) as usize);
    //     let mut path = Vec::with_capacity(k as usize);
    //     Self::combine_impl(&mut result, &mut path, n, 1, k);
    //     result
    // }
    // fn combine_impl(result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, n: i32, start: i32, k: i32)  {
    //     if k == 0 {
    //         result.push(path.clone());
    //         return;
    //     }
    //     for x in start..=(n-k+1) {
    //         path.push(x);
    //         Self::combine_impl(result, path, n, x+1, k-1);
    //         path.pop();
    //     }
    // }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity((n*k)as usize);
        let mut path: Vec<i32> = (1..=k).collect();
        let k = k as usize;
        loop {
            result.push(path.clone());
            let mut i = k;
            while i > 0 {
                i -= 1;
                let max_value = n - k as i32 + i as i32 + 1;
                if path[i] < max_value { break; }
            }
            if path[i] == n - k as i32 + i as i32 + 1 { break; }
            path[i] += 1;
            for j in i + 1..k { path[j] = path[j - 1] + 1; }
        }
        result
    }
}
// @lc code=end

