use super::Solution;

impl Solution {
    pub fn calculate_score(a: i32, b: i32) -> i32 {
        let s1 = a.count_ones();
        let s2 = b.count_ones();

        if s1 >= s2 {
            (s1 * s2) as i32
        } else {
            (s2 / s1) as i32
        }
    }
}
