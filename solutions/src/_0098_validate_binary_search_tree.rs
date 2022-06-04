use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = vec![];
    let mut limit = std::i64::MIN;

    // Stack all elements for left side inorder traversal
    let mut cursor = root.clone();
    while cursor.is_some() || !stack.is_empty() {
        while let Some(node) = cursor {
            stack.push(Rc::clone(&node));
            match node.borrow().left.as_ref() {
                Some(l) => cursor = Some(l.clone()),
                None => cursor = None,
            };
        }
        cursor = stack.pop();

        match cursor {
            Some(node) => {
                if node.borrow().val as i64 <= limit {
                    return false;
                }
                limit = node.borrow().val as i64;
                cursor = match &node.borrow().right {
                    Some(r) => Some(Rc::clone(r)),
                    None => None,
                };
            }
            None => break,
        };
    }

    true
}

#[cfg(test)]
#[test]
fn test_1() {
    let mut root = TreeNode::new(2);
    let left = TreeNode::new(1);
    let right = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(root)))), true);
}

#[test]
fn test_2() {
    let mut root = TreeNode::new(5);
    let left = TreeNode::new(1);
    let mut right = TreeNode::new(4);
    let right_left = TreeNode::new(3);
    let right_right = TreeNode::new(6);
    right.right = Some(Rc::new(RefCell::new(right_right)));
    right.left = Some(Rc::new(RefCell::new(right_left)));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
}

#[test]
fn test_3() {
    let mut root = TreeNode::new(5);
    let left = TreeNode::new(4);
    let mut right = TreeNode::new(6);
    let right_left = TreeNode::new(3);
    let right_right = TreeNode::new(7);
    right.right = Some(Rc::new(RefCell::new(right_right)));
    right.left = Some(Rc::new(RefCell::new(right_left)));
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    assert_eq!(is_valid_bst(Some(Rc::new(RefCell::new(root)))), false);
}
