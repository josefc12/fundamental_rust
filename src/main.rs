
mod binary_tree;
fn main() {
    
    //Example usage of the binary_tree
    let mut tree: binary_tree::Tree<&'static str, f32> = binary_tree::Tree::new();

    tree.set("first key", 12.65);
    tree.set("second key", 99.999);
    tree.set("third key", -128.5);
    tree.set("fourth key", 67.21);

    println!("tree.get_ref(\"third key\") is {}", match tree.get_ref("third key") {
        Err(_) => {println!("Invalid!"); &0.0},
        Ok(x) => x,
    });

}
