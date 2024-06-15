//This is the structure of Node.
struct TreeNode<K, V> where K:PartialOrd + PartialEq { //Accept only those types, that implement PartialOrd and PartialEq
    key: K,
    value: V,
    lesser: Option<Box<TreeNode<K, V>>>, //Here lies another TreeNode that would hold a value directly lesser than this self.
    greater: Option<Box<TreeNode<K, V>>>, //Here lies a TreeNode that would be directly greater than this self.
}

//Implement the set function of the TreeNode.
impl<K, V> TreeNode<K, V> where K:PartialOrd + PartialEq {
    fn set(&mut self, key: K, value: V) {
        //If the passed key finds this TreeNode, set its value to the passed value.
        if key == self.key {
            self.value = value;
        }
        //If the key is lesser than this tree node, proceed to the TreeNode held in the lesser parameter.
        else if key < self.key {
            match self.lesser {
                //If the node does not have a lesser value at all...
                None => {
                    self.lesser = Some(
                        //... create a new TreeNode and assign it appropriate values
                        Box::new(TreeNode {key, value, lesser: None, greater: None})
                    );
                },
                //If it does have a lesser TreeNode, run the set function on that.
                Some(ref mut lesser) => {
                    lesser.set(key, value);
                }
            }
        //Else if the key is greater than the key of self, look into the greater parameter.
        } else {
            match self.greater { 
                //If there's no greater TreeNode, create one
                None => {
                    self.greater = Some(
                        Box::new(TreeNode {key, value, lesser: None, greater: None})
                    );
                }
                //If there is, run its set function.
                Some(ref mut greater) => {
                    greater.set(key, value);
                }
            }
        }
    }

    fn get_ref(&self, key: K) -> Result<&V, String> {
        if key == self.key {
            return Ok(&self.value);
        }
        else if key < self.key {
            match self.lesser {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref lesser) => {
                    return lesser.get_ref(key);
                }
            }
        }
        else {
            match self.greater {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref greater) => {
                    return greater.get_ref(key);
                }
            }
        }
    }

    fn get_mut(&mut self, key: K) -> Result<&mut V, String> {
        if key == self.key {
            return Ok(&mut self.value);
        }
        else if key < self.key {
            match self.lesser {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref mut lesser) => {
                    return lesser.get_mut(key);
                }
            }
        }
        else {
            match self.greater {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref mut greater) => {
                    return greater.get_mut(key);
                }
            }
        }
    }
}

//Structure of a tree. What if you want to say that you have a tree that is empty but you have no TreeNodes? Use this.
pub struct Tree<K, V> where K: PartialOrd + PartialEq { 
    root: Option<Box<TreeNode<K, V>>>,
}

impl<K, V> Tree<K, V> where K: PartialOrd + PartialEq {
    
    pub fn new() -> Tree<K, V> {
        Tree {root:None}
    }

    pub fn set(&mut self, key: K, value: V) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode {key, value, lesser: None, greater: None}));
            }
            Some(ref mut root) => {
                root.set(key, value);
            }
        }
    }

    pub fn get_ref(&self, key :K) -> Result<&V, String> {
        match self.root {
            None => {
                return Err("No such key".to_string());
            }
            Some(ref root) => {
                return root.get_ref(key);
            }
        }
    }

    pub fn get_mut(&mut self, key: K) -> Result<&mut V, String> {
        match self.root {
            None => {
                return Err("No such key".to_string());
            }
            Some(ref mut root) => {
                return root.get_mut(key);
            }
        }
    }
}