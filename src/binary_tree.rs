use std::{cell::RefCell, fmt::Display, io::{Error, ErrorKind}, rc::Rc};

type Node<K,V> = Option<Rc<RefCell<TreeNode<K,V>>>>;

struct TreeNode<K,V> where K: PartialOrd + PartialEq, V: Display + Copy {
    key: K,
    value: V,
    left: Node<K,V>,
    right: Node<K,V>,
}

pub struct Tree<K,V> where K: PartialOrd + PartialEq, V: Display + Copy {
    root: Node<K,V>,
}

impl<K, V> Tree<K, V> where K: PartialOrd + PartialEq, V: Display + Copy{

    pub fn new() -> Tree<K, V> {
        return Tree {root: None }
    }

    pub fn add(&mut self, key: K, value: V) {

        match self.root {
            None=> {
                self.root = Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode{ key: key, value: value, left: None, right: None }
                        )
                    )
                )
            },
            Some(ref mut node)=> { 

                node.borrow_mut().add(key,value);

             }
        }

    }

    pub fn get_ref(&self, key:K) -> Result<V, Error> {
        
        match self.root {
            None=> { Err(Error::new(ErrorKind::Other, "No such key exists.")) },
            Some(ref node)=> { node.borrow().get_ref(key) }
        }

    }

}

impl<K,V> TreeNode<K,V> where K: PartialOrd + PartialEq, V: Display + Copy {

    fn add(&mut self,key:K,value:V){

        //Value gets overriden at a key that already exists.
        if key == self.key  {
            self.value = value;
        }
        else if key > self.key {
            match self.right {
                None=> {
                    self.right = Some(
                        Rc::new(
                            RefCell::new(
                                TreeNode{ key: key, value: value, left: None, right: None }
                            )
                        )
                    )
                },
                Some(ref mut node)=> { 
                    node.borrow_mut().add(key, value);
                }
            }
            
        }
        else if key < self.key {
            match self.left {
                None=> {
                    self.left = Some(
                        Rc::new(
                            RefCell::new(
                                TreeNode{ key: key, value: value, left: None, right: None }
                            )
                        )
                    )
                },
                Some(ref mut node)=> { 
                    node.borrow_mut().add(key, value);
                }
            }
        }
    }

    fn get_ref(&self, key: K) -> Result<V, Error> {

        if key == self.key {
            Ok(self.value)
        }
        else if key > self.key {
            match self.right {
                None=> { Err(Error::new(ErrorKind::Other, "No such key exists.")) },
                Some(ref node)=> { node.borrow().get_ref(key) }
            }
        }
        else if key < self.key {
            match self.left {
                None=> { Err(Error::new(ErrorKind::Other, "No such key exists.")) },
                Some(ref node)=> { node.borrow().get_ref(key) }
            }
        } else {
            Err(Error::new(ErrorKind::Other, "Something unexpected happened."))
        }

    }


    
}

