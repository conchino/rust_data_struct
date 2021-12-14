use crate::signal_list::linkedlist_box::*;


pub fn test1(){
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let mut node = ListNode::vec_to_list(v);
    node.show();
    // println!("{}", node.get_next().unwrap()
    //                    .get_next().unwrap()
    //                    .get_next().unwrap().val);
    node = node.add_head(123);
    node.show();
}


pub fn test2(){
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let mut node = ListNode::vec_to_list(v);
    node.show();
    node = node.add_at_index(3, 11);
    node.show();
}

pub fn test3(){
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let mut node = ListNode::vec_to_list(v);
    node.pop();
    node.push(8);
    node.show();
    node = node.delete_at_index(2);
    node.show();
}

pub fn test4(){
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let mut node = ListNode::vec_to_list(v);
    node.show();
    node.remove(1);
    node.show();
}

pub fn test5(){
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let mut node = ListNode::vec_to_list(v);
    node.show();
    node.insert(1, 11);
    node.show();
}

pub fn test6(){
    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    let mut node = ListNode::vec_to_list(v1);
    node.show();

    node.update(0, 12);
    node.update(1, 23);
    node.show();
}