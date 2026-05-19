/*
 * @lc app=leetcode.cn id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
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
    // pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if root.is_none() { return 0; }
    //     let root = root.clone().unwrap();
    //     (match (root.borrow().left.clone(), root.borrow().right.clone()) {
    //         (Some(l), Some(r)) => i32::min(Self::min_depth(Some(l)), Self::min_depth(Some(r))),
    //         (Some(l), None) => Self::min_depth(Some(l)),
    //         (None, Some(r)) => Self::min_depth(Some(r)),
    //         (None, None) => 0
    //     }) + 1
    // }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        if root.is_none() { return depth; }
        let mut que = VecDeque::new();
        que.push_back(root.unwrap());
        while !que.is_empty() {
            depth += 1;
            let len = que.len();
            for _ in 0..len {
                let x = que.pop_front().unwrap();
                match (x.borrow().left.clone(), x.borrow().right.clone()) {
                    (Some(l), Some(r)) => {
                        que.push_back(l);
                        que.push_back(r);
                    }
                    (Some(l), None) => que.push_back(l),
                    (None, Some(r)) => que.push_back(r),
                    (None, None) => return depth,
                }
            }
        }

        depth
    }
}
// @lc code=end

