use std::rc::Rc;
use std::cell::RefCell;

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


struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}


impl BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = BSTIterator{stack: Vec::new()};
        iter.push_left_nodes(root);
        iter
    }

    fn push_left_nodes(&mut self, node: Option<Rc<RefCell<TreeNode>>>){
        let mut current = node;
        while let Some(node) = current {
            self.stack.push(Rc::clone(&node));
            current = node.borrow().left.as_ref().map(|n| Rc::clone(n));
        }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let node_ref = node.borrow();
        if let Some(right) = &node_ref.right{
            self.push_left_nodes(Some(Rc::clone(right)));
        }
        node_ref.val
    }

    fn has_next(&mut self) -> bool {
        !self.stack.is_empty()
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode{
        val: 7,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode{
            val: 15,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
        }))),
    })));

    let mut iterator = BSTIterator::new(root);
    println!("{}",iterator.next());    // return 3
    println!("{}",iterator.next());    // return 7
    println!("{}",iterator.has_next()); // return True
    println!("{}",iterator.next());    // return 9
    println!("{}",iterator.has_next()); // return True
    println!("{}",iterator.next());    // return 15
    println!("{}",iterator.has_next()); // return True
    println!("{}",iterator.next());    // return 20
    println!("{}",iterator.has_next()); // return False
}
