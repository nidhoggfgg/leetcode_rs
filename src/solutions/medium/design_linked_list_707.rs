/*
 * @lc app=leetcode.cn id=707 lang=rust
 *
 * [707] Design Linked List
 */

// @lc code=start
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;

pub struct MyLinkedList {
    head: Link,
    tail: Link,
    len: usize,
}

struct Node {
    val: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            prev: None,
            next: None,
        }))
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
impl MyLinkedList {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn node_at(&self, index: usize) -> Link {
        if index >= self.len {
            return None;
        }

        if index < self.len / 2 {
            let mut cur = self.head.clone();
            for _ in 0..index {
                cur = cur.unwrap().borrow().next.clone();
            }
            cur
        } else {
            let mut cur = self.tail.clone();
            for _ in index + 1..self.len {
                cur = cur
                    .unwrap()
                    .borrow()
                    .prev
                    .as_ref()
                    .and_then(Weak::upgrade);
            }
            cur
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }

        self.node_at(index as usize)
            .map(|node| node.borrow().val)
            .unwrap_or(-1)
    }

    pub fn add_at_head(&mut self, val: i32) {
        let node = Node::new(val);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(old_head);
                self.head = Some(node);
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let node = Node::new(val);

        match self.tail.take() {
            Some(old_tail) => {
                node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node);
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
        }

        self.len += 1;
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
            return;
        }

        let index = index as usize;
        if index > self.len {
            return;
        }
        if index == self.len {
            self.add_at_tail(val);
            return;
        }

        let next = self.node_at(index).unwrap();
        let prev = next
            .borrow()
            .prev
            .as_ref()
            .and_then(Weak::upgrade)
            .unwrap();
        let node = Node::new(val);

        node.borrow_mut().prev = Some(Rc::downgrade(&prev));
        node.borrow_mut().next = Some(next.clone());
        prev.borrow_mut().next = Some(node.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&node));

        self.len += 1;
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index as usize >= self.len {
            return;
        }

        let target = self.node_at(index as usize).unwrap();
        let (prev, next) = {
            let mut target = target.borrow_mut();
            (
                target.prev.take().and_then(|prev| prev.upgrade()),
                target.next.take(),
            )
        };

        match prev.as_ref() {
            Some(prev) => prev.borrow_mut().next = next.clone(),
            None => self.head = next.clone(),
        }

        match next.as_ref() {
            Some(next) => next.borrow_mut().prev = prev.as_ref().map(Rc::downgrade),
            None => self.tail = prev,
        }

        self.len -= 1;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leetcode_example() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);

        assert_eq!(list.get(1), 2);

        list.delete_at_index(1);

        assert_eq!(list.get(1), 3);
    }

    #[test]
    fn test_delete_head_and_tail() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);

        list.delete_at_index(0);
        assert_eq!(list.get(0), 2);

        list.delete_at_index(1);
        assert_eq!(list.get(0), 2);
        assert_eq!(list.get(1), -1);
    }
}
