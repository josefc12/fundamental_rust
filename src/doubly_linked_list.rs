use std::{borrow::Borrow, cell::RefCell, fmt::Display, rc::Rc};


type DoublyListNodeAlias<V> = Option<Rc<RefCell<DoublyListNode<V>>>>;

pub struct DoublyListNode<V> where V: PartialOrd + PartialEq + Display + Copy{
    value: V,
    prev: DoublyListNodeAlias<V>,
    next: DoublyListNodeAlias<V>
}
pub struct DoublyLinkedList<V> where V: PartialOrd + PartialEq + Display + Copy{
    length: i64,
    head: DoublyListNodeAlias<V>,
    tail: DoublyListNodeAlias<V>
}

impl<V> DoublyLinkedList<V> where V: PartialOrd + PartialEq + Display + Copy{

    pub fn new() -> Self {
        return DoublyLinkedList {length: 0, head: None, tail: None};
    }

    pub fn get_length(&self) -> i64 {
        return self.length;
    }
    pub fn get_head(&self) -> DoublyListNodeAlias<V> {
        return self.head.clone();
    }
    pub fn get_tail(&self) -> DoublyListNodeAlias<V> {
        return self.tail.clone();
    }

    pub fn append_front(&mut self,value: V) {
        match self.head {
            None => {
                self.head = Some(Rc::new(RefCell::new(DoublyListNode {value: value,prev: None , next: None})));
                self.tail = self.head.clone();
                self.length +=1;
            },
            Some(ref mut node) => {
                let new_node = Some(Rc::new(RefCell::new(DoublyListNode {value: value,prev: None , next: Some(node.clone())})));
                node.borrow_mut().prev = new_node.clone();
                self.head = new_node.clone();
                self.length +=1;
            }
        }
    }

    pub fn append_end(&mut self,value: V) {

        let new_node = Some(Rc::new(RefCell::new(DoublyListNode {value: value, prev: None, next: None})));

        match self.tail {
            None => {
                self.tail = new_node.clone();
                self.head = new_node.clone();
                self.length +=1;
            },
            Some(ref node) => {
                let new_tail = Some(Rc::new(RefCell::new(DoublyListNode {value: value, prev: Some(node.clone()), next: None})));
                node.borrow_mut().next = new_tail.clone();
                self.tail = new_tail.clone();
                self.length +=1;
            }
        }
    }
}

impl<V> DoublyListNode<V> where V: PartialOrd + PartialEq + Display + Copy{

    pub fn get_value(&self) -> V{
        return self.value.clone();
    }
}