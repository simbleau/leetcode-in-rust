use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut buffer = vec![];

    let mut stack = vec![];
    let mut cursor = root.clone();
    while cursor.is_some() || !stack.is_empty() {
        // If cursor is some, keep moving left, pushing to stack
        while let Some(node) = cursor {
            stack.push(Rc::clone(&node));
            cursor = match &node.borrow().left {
                Some(left) => Some(Rc::clone(left)),
                None => None,
            };
        }

        // Get last item from the stack
        let next = stack.pop().unwrap();
        buffer.push(next.borrow().val);

        // Move right if we can
        cursor = match &next.borrow().right {
            Some(right) => Some(Rc::clone(right)),
            None => None,
        };
    }

    buffer
}

#[cfg(test)]
#[test]
fn test_1() {
    let mut root = TreeNode::new(1);
    let mut right = TreeNode::new(2);
    let right_left = TreeNode::new(3);
    right.left = Some(Rc::new(RefCell::new(right_left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    assert_eq!(
        inorder_traversal(Some(Rc::new(RefCell::new(root)))),
        vec![1, 3, 2]
    );
}
