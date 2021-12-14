// use crate::linkedlist_Rc::*;
use crate::signal_list::linkedlist_Rc::*;


pub fn test1(){
  let mut node: ListNode<i32> = ListNode::new();
  for _i in 0..10 {
      node.add_head(_i + 1);
  }
  println!("legnth of node: {}", node.len);
  node.show();
}


pub fn test2(){
  let mut node: ListNode<i32> = ListNode::new();
  node.push_back(1);
  node.show();
  node.push_back(2);
  node.push_back(3);
  node.push_back(4);
  node.show();
}


pub fn test3(){
  let mut node = ListNode::vec_to_list(vec![1, 2, 3, 4, 5, 6].as_ref());
  node.show();

  if let Some(ref x) = node.get_last() {
      println!("{}", x.as_ref().borrow().val);
  }

  // get 获取指定索引位置元素
  for _i in 0..node.len as isize {
      if let Some(x) = node.get(_i) {
          print!("{} ", &x.as_ref().borrow().val);
      }
  }
  println!();
  println!("{}", node.get(node.len as isize - 2).unwrap().as_ref().borrow().val);
}


pub fn test4(){
  let mut node = ListNode::vec_to_list(vec![1, 2, 3, 4, 5, 6].as_ref());
  node.pop();
  node.show();
  for _i in 0..3 {
      node.pop();
  }
  node.show();
}

pub fn test5(){
  let mut node = ListNode::vec_to_list(vec![1, 2, 3, 4, 5, 6].as_ref());
  node.pop_head();
  node.pop_head();
  node.pop_head();
  node.show();
}

pub fn test6(){
  let mut node = ListNode::vec_to_list(vec![1, 2, 3, 4, 5, 6].as_ref());
  node.show();
  node.remove(2);
  node.show();
}

pub fn test7(){
  let mut node = ListNode::vec_to_list(vec![1, 2, 3, 4, 5, 6].as_ref());
  node.insert(1, 123);
  node.show();
}