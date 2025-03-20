// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_node = ListNode::new(0);
    let mut current = &mut new_node;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0{
        let sum = carry
            + l1.as_ref().map_or(0, |node| node.val)
            + l2.as_ref().map_or(0, |node| node.val);
        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        l1 = l1.and_then(|node| node.next);
        l2 = l2.and_then(|node| node.next);
    }


    new_node.next
}

fn main() {
   let l1 = Some(Box::new(ListNode{
       val: 2,
       next: Some(Box::new(ListNode{
           val: 4,
           next: Some(Box::new(ListNode::new(3)))
       }))
   }));

    let l2 = Some(Box::new(ListNode{
        val: 5,
        next: Some(Box::new(ListNode{
            val: 6,
            next: Some(Box::new(ListNode::new(4)))
        }))
    }));

    let result = add_two_numbers(l1,l2);

    println!("{:?}", result)
}
