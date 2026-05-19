/*
 * @lc app=leetcode.cn id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
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
    // pub fn preorder_traversal(root: Option<Node>) -> Vec<i32> {
    //     let mut result = Vec::new();
    //     Self::traversal(root, &mut result);
    //     result
    // }
    // fn traversal(node: Option<Node>, result: &mut Vec<i32>) {
    //     if node.is_none() {return;}
    //     result.push(node.as_ref().unwrap().borrow().val);
    //     Self::traversal(node.as_ref().unwrap().borrow().left.clone(), result);
    //     Self::traversal(node.as_ref().unwrap().borrow().right.clone(), result);
    // }

    pub fn preorder_traversal(root: Option<Node>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {return result};
        let mut stack = Vec::new();
        stack.push(root.unwrap());
        while let Some(n) = stack.pop() {
            result.push(n.borrow().val);
            if let Some(x) = n.borrow().right.clone() {
                stack.push(x);
            }
            if let Some(x) = n.borrow().left.clone() {
                stack.push(x);
            }
        }
        result
    }
}
// @lc code=end

