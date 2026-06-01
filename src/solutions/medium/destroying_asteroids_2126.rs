/*
 * @lc app=leetcode.cn id=2126 lang=rust
 *
 * [2126] Destroying Asteroids
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut mass = mass as i64;
        asteroids.sort_unstable();
        for x in asteroids {
            if x as i64 > mass { return false; }
            mass+=x as i64;
        }
        true
    }
}
// @lc code=end

