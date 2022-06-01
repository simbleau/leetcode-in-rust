use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

pub fn recurse(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root) = root {
        if let (Some(ref left), Some(ref right)) =
            (&root.borrow().left, &root.borrow().right)
        {
            let node_val = root.borrow().val;
            //let left_val = left.borrow().val;
            //let right_val = right.borrow().val;
            if p > node_val && q > node_val {
                // Recurse right
                let right = Some(Rc::clone(right));
                return recurse(&right, p, q);
            } else if p < node_val && q < node_val {
                // Recurse left
                let left = Some(Rc::clone(left));
                return recurse(&left, p, q);
            }
        }
        // This is the lowest common ancestor
        return Some(Rc::new(RefCell::new(TreeNode::new(root.borrow().val))));
    } else {
        return None;
    }
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if p.is_none() || q.is_none() || root.is_none() {
        return root;
    }

    let p_val = p.unwrap().borrow_mut().val;
    let q_val = q.unwrap().borrow_mut().val;
    return recurse(&root, p_val, q_val);
}
#[cfg(test)]
#[test]
fn test_1() {
    let mut l = TreeNode::new(2);
    let l_l = TreeNode::new(0);
    let mut l_r = TreeNode::new(4);
    let l_r_l = TreeNode::new(3);
    let l_r_r = TreeNode::new(5);
    l_r.left = Some(Rc::new(RefCell::new(l_r_l)));
    l_r.right = Some(Rc::new(RefCell::new(l_r_r)));
    l.left = Some(Rc::new(RefCell::new(l_l)));
    l.right = Some(Rc::new(RefCell::new(l_r)));

    let mut r = TreeNode::new(8);
    let r_l = TreeNode::new(7);
    let r_r = TreeNode::new(9);
    r.left = Some(Rc::new(RefCell::new(r_l)));
    r.right = Some(Rc::new(RefCell::new(r_r)));

    let mut root = TreeNode::new(6);
    root.left = Some(Rc::new(RefCell::new(l)));
    root.right = Some(Rc::new(RefCell::new(r)));

    let root = Some(Rc::new(RefCell::new(root)));
    let p = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let q = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let expected = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    assert_eq!(lowest_common_ancestor(root, p, q), expected);
}
