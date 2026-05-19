/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] Invert Binary Tree
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
use std::mem::swap;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert(root.clone());
        root
    }

    fn invert(node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = &node {
            {
                let mut borrowed_node = n.borrow_mut();
                let TreeNode { left, right, ..} = &mut *borrowed_node;
                swap(left, right);
            }
            Self::invert(n.borrow().left.clone());
            Self::invert(n.borrow().right.clone());
        }
    }
}
// @lc code=end

