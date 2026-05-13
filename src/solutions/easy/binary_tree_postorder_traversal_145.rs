/*
 * @lc app=leetcode.cn id=145 lang=rust
 *
 * [145] Binary Tree Postorder Traversal
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
    pub fn postorder_traversal(root: Option<Node>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::traversal(root, &mut result);
        result
    }

    fn traversal(node: Option<Node>, result: &mut Vec<i32>) {
        if node.is_none() {return;}
        Self::traversal(node.as_ref().unwrap().borrow().left.clone(), result);
        Self::traversal(node.as_ref().unwrap().borrow().right.clone(), result);
        result.push(node.as_ref().unwrap().borrow().val);
    }
}
// @lc code=end

