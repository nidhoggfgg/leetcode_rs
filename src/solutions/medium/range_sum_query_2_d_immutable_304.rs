/*
 * @lc app=leetcode.cn id=304 lang=rust
 *
 * [304] Range Sum Query 2D - Immutable
 */

// @lc code=start
pub struct NumMatrix{p:Vec<Vec<i32>>}
impl NumMatrix {
    pub fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        for i in 0..matrix.len(){
            for j in 1..matrix[i].len(){
                matrix[i][j]+=matrix[i][j-1];
            }
        }
        Self{p:matrix}
    }
    
    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut ans=0;
        let (row1,col1,row2,col2)=(row1 as usize,col1 as usize,row2 as usize,col2 as usize);
        for row in row1..=row2{
            if col1>0{ans+=self.p[row][col2]-self.p[row][col1-1]}
            else {ans+=self.p[row][col2]}
        }
        ans
    }
}
// @lc code=end

