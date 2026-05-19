/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
pub struct Solution;

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    // pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut depth = 0;
    //     if root.is_none() { return depth; }
    //     let mut que= VecDeque::new();
    //     que.push_back(root.unwrap());
    //     while !que.is_empty() {
    //         let len = que.len();
    //         for _ in 0..len {
    //             let x = que.pop_front().unwrap();
    //             if let Some(n) = x.borrow().left.clone() { que.push_back(n); }
    //             if let Some(n) = x.borrow().right.clone() { que.push_back(n); }
    //         }
    //         depth += 1;
    //     }
    //     depth
    // }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let left_depth = Self::max_depth(root.clone().unwrap().borrow().left.clone());
        let right_depth = Self::max_depth(root.clone().unwrap().borrow().right.clone());
        i32::max(left_depth, right_depth) + 1
    }
}
// @lc code=end

