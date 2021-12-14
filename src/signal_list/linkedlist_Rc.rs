use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::{fmt::Display};

// 链表节点
#[derive(Clone,PartialEq, Debug)]
pub struct Node<T: Copy + Display + Debug>{
    pub val: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

impl<T: Copy + Display + Debug> Node<T>{

    pub fn new(value: T) -> Node<T>{
        Node{
            val: value,
            next: None
        }
    }

    pub fn get_next(&mut self) -> Option<Rc<RefCell<Node<T>>>>{
        if let Some(ref mut x) = self.next.borrow_mut() {
            return Some(x.clone());
        }
        None
    }

    pub fn set_next(&mut self, next: Option<Rc<RefCell<Node<T>>>>) -> () {
        if next.is_none() {
            self.next = None;
            return
        }
        let next_node = next.as_ref().unwrap();
        self.next = Some(Rc::clone(next_node));
    }
}


// 链表头节点
#[derive(Clone,PartialEq, Debug)]
pub struct ListNode<T: Copy + Display + Debug>{
    head: Option<Rc<RefCell<Node<T>>>>,
    pub len: usize
}


impl<T: Copy + Display + Debug> ListNode<T>{

    pub fn new() -> ListNode<T>{
        ListNode { len: 0, head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // 头部插入
    pub fn add_head(&mut self, ele: T){
        let new_node = Rc::new(RefCell::new(Node::new(ele)));
        if let Some(head) = self.head.take() {
            new_node.as_ref().borrow_mut().next = None;
            new_node.as_ref().borrow_mut().next = Some(head);
        }
        self.head = Some(new_node);
        self.len += 1;
    }

    // 尾部插入
    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        if let Some(head_ref) = self.head.borrow() {
            // println!("linklist not None!!");
            let mut cur_node = Rc::clone(head_ref);
            while let Some(ref next_node) = Rc::clone(&cur_node).as_ref().borrow().next {
                cur_node = next_node.clone();
            }
            cur_node.as_ref().borrow_mut().next = Some(new_node);
            self.len += 1;
        }else{
            // println!("linklist is None...");
            self.head = Some(new_node);
            self.len += 1;
        };

    }

    // 获取最后一个元素
    pub fn get_last(&self) -> Option<Rc<RefCell<Node<T>>>> {
        if let Some(head_ref) = self.head.borrow() {
            let mut cur_node = head_ref.clone();
            while let Some(ref next_node) = Rc::clone(&cur_node).as_ref().borrow_mut().next {
                cur_node = next_node.clone();
            }
            return Some(cur_node);
        }
        println!("linklist is empty!");
        None
    }

    // 获取指定索引位置元素
    pub fn get(&self, index: isize) -> Option<Rc<RefCell<Node<T>>>> {
        if let Some(head_ref) = self.head.borrow() {
            let mut cur_node = head_ref.clone();
            if index <= 0 {
                return Some(cur_node);
            }
            if index >= self.len as isize {
                return self.get_last();
            }
            let mut lens = index;
            while lens >= 1 {
                if let Some(ref next_node) = Rc::clone(&cur_node).as_ref().borrow_mut().next {
                    cur_node = next_node.clone();
                }else {
                    return None;
                }
                lens -= 1; 
            }
            return Some(cur_node);
        }
        println!("linklist is empty!");
        None
    }

    // 获取指定位置被option包装的值
    pub fn index(&self, index: isize) -> Option<T> {
        if let Some(res) = self.get(index) {
           return Some(res.as_ref().borrow().val);
        }
        None
    }
 
    // 删除末尾节点
    pub fn pop(&mut self){
        if self.len == 0 {
            println!("linklist is empty!");
            return;
        }
        if self.len <= 1 {
            self.head = None;
            self.len -= 1;
            return;
        }
        if let Some(head_ref) = self.head.borrow() {
            let mut cur_node = Rc::clone(head_ref);
            let mut lens = self.len - 2;
            while lens > 0 {
                if let Some(ref next_node) = Rc::clone(&cur_node).as_ref().borrow_mut().next {
                    cur_node = next_node.clone();
                }else {
                    println!("error!");
                    return;
                }
                lens -= 1;
            }
            cur_node.as_ref().borrow_mut().next = None;
            self.len -= 1;
        }
    }

    // 删除首节点
    pub fn pop_head(&mut self){
        if self.is_empty() {
            println!("linklist is empty!");
            return;
        }
        if let Some(head_ref) = self.head.take() {
            let tmp = head_ref.as_ref().borrow().clone().next;
            self.head = tmp;
        }
    }

    // 删除指定位置节点
    pub fn remove(&mut self, index: usize){
       if self.len < 1 || index >= self.len {
           println!("链表为空或删除索引位置错误");
           return;
       } 
       if index == 0 {
           let head_node = self.head.take().unwrap();
           let head_next = &head_node.as_ref().borrow().next;
           if head_next.is_none() {
               self.head = None;
               self.len -= 1;
               return ;
           }
           let next_node = head_next.as_ref().unwrap();
           self.head = Some(Rc::clone(next_node));
           self.len -= 1;
           return ;
       }else {
           // 获取首节点
            let mut cur_node = Rc::clone(self.head.as_ref().unwrap());
            for _i in 0..index - 1 {
                let next = Rc::clone(cur_node.as_ref().borrow().next.as_ref().unwrap());
                cur_node = next;
            }
            let mut curr_node = cur_node.as_ref().borrow_mut();
            let new_option = {
                if curr_node.next.is_none() {
                    None
                } else {
                    let next_node = curr_node.next.as_ref().unwrap();
                    let next_node = &next_node.as_ref().borrow().next;
                    if next_node.is_none() {
                        None
                    } else {
                        Some(Rc::clone(next_node.as_ref().unwrap()))
                    }
                }
            };
            curr_node.next = Some(Rc::clone(new_option.as_ref().unwrap()));
            self.len -= 1;
       }
    }

    // 在指定位置插入值
    pub fn insert(&mut self, index: usize, value: T){
        if index <= 0 {
            self.add_head(value);
        }else if index >= self.len {
            self.push_back(value);
        }else{
            let mut new_node = Node::new(value);
            let mut curr = Rc::clone(self.head.as_ref().unwrap());
            for _i in 0..(index - 1) {
                let _node = Rc::clone(&curr);
                curr = Rc::clone(&_node.as_ref().borrow_mut().next.as_ref().unwrap());
            }
            let post = if let Some(post_node) = &curr.as_ref().borrow_mut().next {
                Some(Rc::clone(post_node))
            } else {
                None
            };
            new_node.set_next(post);
            curr.as_ref().borrow_mut().set_next(Some(Rc::new(RefCell::new(new_node))));
            self.len += 1;
        }
    }

    // 打印链表
    pub fn show(&self){
        if let Some(head_ref) = self.head.borrow() {
            let mut cur_node = head_ref.clone();
            print!("{} ", cur_node.as_ref().borrow_mut().val);   // mut 只能用borrow_mut()
            while let Some(ref next_node) = Rc::clone(&cur_node).as_ref().borrow_mut().next {
                cur_node = next_node.clone();
                // print!("{} ", cur_node.borrow_mut().val);s
                print!("{} ", cur_node.as_ref().borrow().val);
            }
            println!();
        }else {
            println!("ListNode is None !");
        }
    }

    // vector转链表
    pub fn vec_to_list(vs: &Vec<T>) -> ListNode<T> {
        let mut node: ListNode<T> = ListNode::new();
        for v in vs {
            node.push_back(v.clone());
        }
        node
    }

}