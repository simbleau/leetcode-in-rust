use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub fn check(
    p: &Option<Rc<RefCell<TreeNode>>>,
    q: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    if p.is_none() && q.is_none() {
        return true;
    }
    if p.is_none() || q.is_none() {
        return false;
    }
    if let (Some(p), Some(q)) = (p, q) {
        if p.borrow().val != q.borrow().val {
            return false;
        }
    }
    // Items are both some, and contain the same value
    true
}

pub fn is_same_tree(
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    // Optimizations
    if p.is_none() && q.is_none() {
        return true;
    } else if !check(&p, &q) {
        return false;
    }

    // Use a stack and BFS
    let mut stack_p = Vec::new();
    let mut stack_q = Vec::new();
    stack_p.push(p);
    stack_q.push(q);

    while !stack_p.is_empty() {
        let item_p = stack_p.pop().unwrap();
        let item_q = stack_q.pop().unwrap();
        if !check(&item_p, &item_q) {
            return false;
        }
        if let (Some(item_p), Some(item_q)) = (item_p, item_q) {
            if !check(&item_p.borrow().left, &item_q.borrow().left) {
                return false;
            };
            if let (Some(p_left), Some(q_left)) =
                (&item_p.borrow().left, &item_q.borrow().left)
            {
                let p_left = Some(Rc::clone(p_left));
                let q_left = Some(Rc::clone(q_left));
                stack_p.push(p_left);
                stack_q.push(q_left);
            }

            if !check(&item_p.borrow().right, &item_q.borrow().right) {
                return false;
            };
            if let (Some(p_right), Some(q_right)) =
                (&item_p.borrow().right, &item_q.borrow().right)
            {
                let p_right = Some(Rc::clone(p_right));
                let q_right = Some(Rc::clone(q_right));
                stack_p.push(p_right);
                stack_q.push(q_right);
            }
        }
    }

    true
}
#[cfg(test)]
#[test]
fn test_1() {
    use crate::TreeNode;

    let mut root1 = TreeNode::new(1);
    let leaf1 = TreeNode::new(2);
    let mut root2 = TreeNode::new(1); // Identical to root1
    let leaf2 = TreeNode::new(2); // Identical to leaf1
    root1.left = Some(Rc::new(RefCell::new(leaf1)));
    root2.right = Some(Rc::new(RefCell::new(leaf2)));

    let root1 = Some(Rc::new(RefCell::new(root1)));
    let root2 = Some(Rc::new(RefCell::new(root2)));
    assert_eq!(is_same_tree(root1, root2), false);
}
