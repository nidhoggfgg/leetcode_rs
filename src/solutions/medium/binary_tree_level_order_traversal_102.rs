/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
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
use std::collections::VecDeque;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut que = VecDeque::new();
        let mut result = Vec::new();
        if root.is_none() { return result; }
        que.push_back(root.unwrap());
        while !que.is_empty() {
            let len = que.len();
            let mut layer = Vec::new();
            for _ in 0..len {
                let x = que.pop_front().unwrap();
                layer.push(x.borrow().val);
                if let Some(l) = x.borrow().left.clone() { que.push_back(l); }
                if let Some(r) = x.borrow().right.clone() { que.push_back(r); }
            }
            result.push(layer);
        }

        result
    }
}
// @lc code=end

