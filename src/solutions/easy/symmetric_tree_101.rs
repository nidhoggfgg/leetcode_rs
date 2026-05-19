/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] Symmetric Tree
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
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn is_symmetric(root: Option<Node>) -> bool {
        if root.is_none() { return true; }
        let root = root.unwrap();
        match (root.borrow().left.clone(), root.borrow().right.clone()) {
            (Some(l), Some(r)) => Self::cmp(l, r),
            (None, None) => true,
            (_, _) => false,
        }
    }

    fn cmp(left: Node, right: Node) -> bool {
        if left.borrow().val != right.borrow().val { return false; }
        match (left.borrow().left.clone(), right.borrow().right.clone()) {
            (Some(l), Some(r)) => if !Self::cmp(l, r) { return false; }
            (None, None) => {},
            (_, _) => return false
        }
        match (left.borrow().right.clone(), right.borrow().left.clone()) {
            (Some(l), Some(r)) => if !Self::cmp(l, r) { return false; }
            (None, None) => {},
            (_, _) => return false
        }

        true
    }
}
// @lc code=end

