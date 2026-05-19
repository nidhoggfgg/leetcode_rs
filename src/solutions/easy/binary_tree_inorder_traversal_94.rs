/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
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
    // pub fn inorder_traversal(root: Option<Node>) -> Vec<i32> {
    //     let mut result = Vec::new();
    //     Self::traversal(root, &mut result);
    //     result
    // }
    // fn traversal(node: Option<Node>, result: &mut Vec<i32>) {
    //     if node.is_none() { return; }
    //     Self::traversal(node.as_ref().unwrap().borrow().left.clone(), result);
    //     result.push(node.as_ref().unwrap().borrow().val);
    //     Self::traversal(node.as_ref().unwrap().borrow().right.clone(), result);
    // }

    pub fn inorder_traversal(root: Option<Node>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {return result;}
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root.clone();
        while !cur.is_none() || !stack.is_empty() {
            if let Some(n) = cur.clone() {
                cur = n.borrow().left.clone();
                stack.push(n);
            } else {
                let x = stack.pop().unwrap();
                result.push(x.borrow().val);
                cur = x.borrow().right.clone();
            }
        }
        result
    }
}
// @lc code=end

