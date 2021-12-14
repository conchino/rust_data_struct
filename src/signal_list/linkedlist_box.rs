use std::{fmt::Display, option, vec};


// 定义链表节点结构体
#[derive(Clone,PartialEq)]
pub struct Node<T: Copy + Display>{
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: Copy + Display> Node<T>{
    // 初始化链表节点
    pub fn new(val: T) -> Node<T>{
        Node{
            val,
            next: None,
        }
    }
    // 获取最后一个节点
    pub fn get_last<'a>(&'a mut self) -> &'a mut Node<T>{
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }
    // 节点操作统一使用Option包装，防止None出现的原因
    pub fn set_next(&mut self, node: Option<Node<T>>) {
        self.next = None;
        if let Some(x) = node {
            self.next = Some(Box::new(x));
        }
    }
    // 获取下一个节点
    pub fn get_next<'a>(&'a mut self) -> Option<&'a mut Self> {
        if let Some(ref mut x) = self.next {
            return Some(x);
        }
        None
    }
    // 链表尾部插入
    pub fn push(&mut self, val: T){
        self.get_last().set_next(Some(Node::new(val)));
    }
    // 根据索引获取变量
    pub fn get<'a>(&'a mut self, index: usize) -> Option<&'a mut Self> {
        if index == 0 {
            return Some(self);
        }
        if let Some(x) = self.get_next() {
            x.get(index - 1)
        } else {
            None
        }
    }
}


// 定义链表头节点结构体
#[derive(Clone)]
pub struct ListNode<T: Copy + Display>{
    len: usize,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy + Display> ListNode<T>{

    pub fn new() -> ListNode<T>{
        ListNode{
            len: 0,
            next: None
        }
    }

    pub fn get_next<'a>(&'a mut self) -> Option<&'a mut Node<T>> {
        if let Some(ref mut x) = self.next {
            // return x.get_next();
            return Some(x);
        }
        None
    }

    // 根据索引获取节点
    pub fn get<'a>(&'a mut self, index: usize) -> Option<&'a mut Node<T>> {
        if index > self.len || index < 0 {
            return None;
        }
        let node = self.get_next().unwrap();
        // if index == 0 {
        //     return Some(node);
        // }
        node.get(index)
    }

    // 获取最后一个节点
    pub fn get_last<'a>(&'a mut self) -> Option<&'a mut Node<T>> {
        if let Some(ref mut x) = self.next {
            Some(x.get_last())
        } else {
            None
        }
    }

    // 根据索引值和输入值更新节点值
    pub fn update(&mut self, index: usize, value: T){
        if let Some(x) = self.get(index) {
            x.val = value;
        }else {
            println!("error, the node is None");
        }
    }

    // 尾部插入元素
    pub fn push(&mut self, elem: T){
        if self.len == 0 {
            self.next = Some(Box::new(Node::new(elem)));
        }else {
            if let Some(ref mut x) = self.get_last() {
                x.push(elem);
            }
        }
        self.len += 1;
    }

    // 删除尾节点
    pub fn pop(&mut self){
        // 仅有一个头节点
        if self.len == 0 {
            return ();
        }
        self.len -= 1;
        let index = self.len;
        self.get(index - 1).unwrap().set_next(None);
    }

    // 获取节点长度
    pub fn len(&self) -> usize {
        self.len
    }

    // 在头部插入节点
    pub fn add_head(mut self, elem: T) -> ListNode<T>{
        if self.len == 0 {
            self.push(elem);
            return self;
        }
        // let new_node = Some(Box::new(Node::new(elem)));
        let mut new_node = Node::new(elem);
        let origin_node = self.next.unwrap();
        new_node.set_next(Some(*origin_node));
        self.next = Some(Box::new(new_node));
        self.len += 1;
        self
    }

    // vector转链表
    pub fn vec_to_list(vct: Vec<T>) -> ListNode<T>{
        let mut head_node = ListNode::new();
        for n in vct {
            head_node.push(n);
        }
        head_node
    }

    // 展示链表节点
    pub fn show(&mut self){
        let len = self.len;
        // 获取除头节点外的第一个节点元素
        let mut node = self.get_next().unwrap();
        for i in 0..len {
            if i != len - 1 {
                print!("{} --> ", node.val);
            }else {
                println!("{}", node.val);
            }
            if node.get_next().is_some() {
                node = node.get_next().unwrap();
            }
            // else {
            //     println!("None");
            // }
        }
    }

    // 在指定位置插入元素 (使用Box不使用引用
    pub fn add_at_index(mut self, index: usize, elem: T) -> ListNode<T>{
        if index > self.len{
            self.push(elem);
        }else if index <= 0 {
            self = self.add_head(elem);
        }else {
            println!("insert_at_index {}", index);
            let mut _length = index - 1;
            let mut head = *self.next.unwrap();
            let mut new_head = ListNode::new();
            new_head.push(head.val);
            while _length > 0 {
                head = match head.next {
                    Some(x) => { new_head.push(x.val); *x},
                    None => break,
                };
                _length -= 1;
            }
            let mut new_node = Node::new(elem);
            new_node.set_next(Some(*head.next.unwrap()));
            // new_head.get_last().unwrap().next = Some(Box::new(new_node));
            new_head.get_last().unwrap().set_next(Some(new_node));
            new_head.len = self.len + 1;
            self = new_head;
        }
        self
    }
    
    // 删除索引位置元素 (使用Box不使用引用
    pub fn delete_at_index(mut self, index: usize) -> ListNode<T>{
        if index <= 0 {
            let new_next = self.next.unwrap().next.unwrap();
            self.next = Some(Box::new(*new_next));
            self.len -= 1;
        }else if index >= self.len {
            self.pop();
        }else {
            println!("delete_at_index {}", index);
            let mut _len = index;
            let mut head = *self.next.unwrap();
            let mut new_head = ListNode::new();
            while _len > 0 {
                new_head.push(head.val);
                head = *head.next.unwrap();
                _len -= 1;
            }
            head = *head.next.unwrap();
            new_head.get_last().unwrap().set_next(Some(head));
            new_head.len = self.len - 1;
            self = new_head;
        }
        self
    }

    // 删除指定位置元素 (使用Box和引用
    pub fn remove(&mut self, index: usize) -> bool{
        if index >= self.len {
            return false;
        }
        match self.next.as_mut() {
            None => false,
            Some(mut cur) => {
                for _ in 0..index - 1 {
                    cur = cur.next.as_mut().unwrap();
                }
                match cur.next.take() {
                    None => false,
                    Some(del_node) => {
                        cur.next = del_node.next;
                        self.len -= 1;
                        true
                    }
                }
            }
        }
    }

    // 在指定位置插入元素 (使用Box和引用
    pub fn insert(&mut self, index: usize, value: T){
        if index > self.len{
            self.push(value);
        }else if index <= 0 {
            // take: 掏空拿出当前元素的值
            let head_node = self.next.take();
            self.next = Some(Box::new(Node::new(value)));
            // 如果直接使用self.next会因为值move而报错，
            // 需要在使用它的时候加一个as_mut,将他作为可变引用, 获取它的修改权
            // 加了as_mut相当于: self -> &mut self
            self.next.as_mut().unwrap().next = head_node;
            self.len += 1;
        }else {
            match self.next.as_mut() {
                None => {
                    self.next = Some(Box::new(Node::new(value)));
                    self.len += 1;
                },
                Some(mut cur) => {
                    for _ in 0..index - 1 {
                        cur = cur.next.as_mut().unwrap();
                    }
                    // 获取下一个节点, 使用take取出它的在Option(Some)中的值
                    let next_node = cur.next.take();
                    cur.next = Some(Box::new(Node::new(value)));
                    cur.next.as_mut().unwrap().next = next_node;
                    self.len += 1;
                }
            }
        }
    }
}
