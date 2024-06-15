use std::{cell::RefCell, fmt::Display, rc::Rc};


type ListNodeAlias<V> = Option<Rc<RefCell<ListNode<V>>>>;

pub struct ListNode<V> where V: PartialOrd + PartialEq + Display + Copy{
    value: V,
    next: ListNodeAlias<V>
}
pub struct LinkedList<V> where V: PartialOrd + PartialEq + Display + Copy{
    length: i64,
    head: ListNodeAlias<V>,
    tail: ListNodeAlias<V>
}

impl<V> LinkedList<V> where V: PartialOrd + PartialEq + Display + Copy{

    pub fn new() -> Self {
        return LinkedList {length: 0, head: None, tail: None};
    }

    pub fn get_length(&self) -> i64 {
        return self.length;
    }
    pub fn get_head(&self) -> ListNodeAlias<V> {
        return self.head.clone();
    }
    pub fn get_tail(&self) -> ListNodeAlias<V> {
        return self.tail.clone();
    }

    pub fn append_front(&mut self,value: V) {
        match self.head {
            None => {
                self.head = Some(Rc::new(RefCell::new(ListNode {value: value, next: None})));
                self.tail = self.head.clone();
                self.length +=1;
            },
            Some(ref node) => {
                self.head = Some(Rc::new(RefCell::new(ListNode {value: value, next: Some(node.clone())})));
                self.length +=1;
            }
        }
    }

    pub fn append_end(&mut self,value: V) {

        let new_node = Some(Rc::new(RefCell::new(ListNode {value: value, next: None})));

        match self.tail {
            None => {
                self.tail = new_node.clone();
                self.head = new_node.clone();
                self.length +=1;
            },
            Some(ref node) => {
                node.borrow_mut().next = new_node.clone();
                self.tail = new_node.clone();
                self.length +=1;
            }
        }
    }
}

impl<V> ListNode<V> where V: PartialOrd + PartialEq + Display + Copy{

    pub fn get_value(&self) -> V{
        return self.value.clone();
    }
}