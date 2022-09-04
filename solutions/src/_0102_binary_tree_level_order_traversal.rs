use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

fn height(level: usize, node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    match node {
        None => level,
        Some(node) => {
            let left = height(level + 1, &node.borrow().left);
            let right = height(level + 1, &node.borrow().right);
            left.max(right)
        }
    }
}

fn traverse(
    results: &mut Vec<Vec<i32>>,
    level: usize,
    node: &Option<Rc<RefCell<TreeNode>>>,
) {
    if let Some(node) = node {
        traverse(results, level + 1, &node.borrow().left);
        results.get_mut(level).unwrap().push(node.borrow().val);
        traverse(results, level + 1, &node.borrow().right);
    }
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut results = vec![];
    let height = height(0, &root);
    for _ in 0..height {
        results.push(vec![]);
    }
    traverse(&mut results, 0, &root);
    results
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

    let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
    assert_eq!(level_order(Some(Rc::new(RefCell::new(root)))), expected);
}

#[test]
fn test_2() {
    let root = TreeNode::new(1);
    let expected = vec![vec![1]];
    assert_eq!(level_order(Some(Rc::new(RefCell::new(root)))), expected);
}

#[test]
fn test_3() {
    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(level_order(None), expected);
}
