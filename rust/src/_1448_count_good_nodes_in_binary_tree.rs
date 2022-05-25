use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn count_good_nodes(node: &Rc<RefCell<TreeNode>>, max: i32) -> i32 {
    let mut good_nodes = 0;

    // Am I a good node?
    if node.borrow().val >= max {
        good_nodes += 1;
    }

    // Are my children good nodes?
    let new_max = std::cmp::max(max, node.borrow().val);
    match node.borrow().left.as_ref() {
        Some(left) => {
            good_nodes += count_good_nodes(left, new_max);
        }
        None => (),
    };
    match node.borrow().right.as_ref() {
        Some(right) => {
            good_nodes += count_good_nodes(right, new_max);
        }
        None => (),
    };

    good_nodes
}

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root) => return count_good_nodes(&root, root.borrow().val),
        None => return 0,
    }
}
#[cfg(test)]
#[test]
fn test_1() {
    //     [3]
    //     / \
    //    1  [4]
    //   /   / \
    // [3]  1  [5]
    let root = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
    }));
    assert_eq!(good_nodes(Some(root)), 4);
}

#[test]
fn test_2() {
    //     [3]
    //     /
    //   [3]
    //   / \
    // [4]  2
    let root = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        }))),
        right: None,
    }));
    assert_eq!(good_nodes(Some(root)), 3);
}

#[test]
fn test_3() {
    // [1]
    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }));
    assert_eq!(good_nodes(Some(root)), 1);
}

#[test]
fn test_4() {
    //  [2]
    //    \
    //    [4]
    //    / \
    //  [8] [10]
    //  /
    // 4
    let root = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: None,
            }))),
        }))),
    }));
    assert_eq!(good_nodes(Some(root)), 4);
}
