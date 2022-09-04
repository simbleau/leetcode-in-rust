use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

fn height(level: i32, node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        None => level,
        Some(node) => {
            let left = height(level + 1, &node.borrow().left);
            let right = height(level + 1, &node.borrow().right);
            left.max(right)
        }
    }
}
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    height(0, &root)
}

#[cfg(test)]
#[test]
fn test_1() {
    let mut root = TreeNode::new(3);
    let left = TreeNode::new(9);
    let mut right = TreeNode::new(20);
    let right_left = TreeNode::new(15);
    let right_right = TreeNode::new(7);
    right.left = Some(Rc::new(RefCell::new(right_left)));
    right.right = Some(Rc::new(RefCell::new(right_right)));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));

    let expected = 3;
    assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), expected);
}

#[test]
fn test_2() {
    let root = TreeNode::new(1);
    let expected = 1;
    assert_eq!(max_depth(Some(Rc::new(RefCell::new(root)))), expected);
}

#[test]
fn test_3() {
    let expected = 0;
    assert_eq!(max_depth(None), expected);
}
