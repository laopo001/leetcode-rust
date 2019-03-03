use crate::base::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(r) = &root {
        if r.borrow().val < val {
            return search_bst(r.borrow().left.clone(), val);
        }
        if r.borrow().val > val {
            return search_bst(r.borrow().right.clone(), val);
        }
    }
    return root;
}
