
mod binary_tree;
mod linked_list;
mod doubly_linked_list;
fn main() {
    
    //Example usage of the binary_tree ---------------------------------------------------
    let mut tree: binary_tree::Tree<&'static str, f32> = binary_tree::Tree::new();

    tree.add("first key", 12.65);
    tree.add("second key", 99.999);
    tree.add("third key", -128.5);
    tree.add("fourth key", 67.21);

    let result:String = match tree.get_ref("second key") {
        Err(e)=> String::from(e.to_string()),
        Ok(v)=> String::from(v.to_string())
    };

    println!("Tree result: {}", &result);
    //------------------------------------------------------------------------------------

    println!("---------------------------------------------------------------");

    //Linked List example ----------------------------------------------------------------
    let mut l_list: linked_list::LinkedList<i32> = linked_list::LinkedList::new();
    
    l_list.append_end(12);
    l_list.append_end(33);
    l_list.append_end(5548);
    l_list.append_front(1);

    let l_head:String = match l_list.get_head() {
        None => {String::from("not found")},
        Some(ref node) => {
            String::from(node.borrow().get_value().to_string())
        }
    };
    let l_tail:String = match l_list.get_tail() {
        None => {String::from("not found")},
        Some(ref node) => {
            String::from(node.borrow().get_value().to_string())
        }
    };

    println!("LinkedList... Node count: {}",&l_list.get_length());
    println!("LinkedList... Head of the list: {}",&l_head);
    println!("LinkedList... Tail of the list: {}",&l_tail);
    //------------------------------------------------------------------------------------

    println!("---------------------------------------------------------------");

    //Doubly Linked List example ---------------------------------------------------------
    let mut d_list: doubly_linked_list::DoublyLinkedList<i32> = doubly_linked_list::DoublyLinkedList::new();

    d_list.append_end(36);
    d_list.append_end(56);
    d_list.append_end(1212);
    d_list.append_front(666);

    let d_head:String = match d_list.get_head() {
        None => {String::from("not found")},
        Some(ref node) => {
            String::from(node.borrow().get_value().to_string())
        }
    };
    let d_tail:String = match d_list.get_tail() {
        None => {String::from("not found")},
        Some(ref node) => {
            String::from(node.borrow().get_value().to_string())
        }
    };

    println!("DoublyLinkedList... Node count: {}",&d_list.get_length());
    println!("DoublyLinkedList... Head of the list: {}",&d_head);
    println!("DoublyLinkedList... Tail of the list: {}",&d_tail);
    //------------------------------------------------------------------------------------
}
