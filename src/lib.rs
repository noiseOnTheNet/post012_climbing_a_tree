#[derive(Debug)]
pub struct Node<T>{
    value : T,
    left : Option<Box<Node<T>>>,
    right : Option<Box<Node<T>>>
}

impl<T> Node<T>{
    fn new(value : T) -> Node<T>{
        Node{
            value : value,
            left : None,
            right : None
        }
    }
}

impl<T> From<Node<T>> for Option<Box<Node<T>>>{
    fn from(value: Node<T>) -> Self {
        Some(Box::new(value))
    }
}

#[derive(Debug,Default)]
pub struct Tree<T>{
    root: Option<Box<Node<T>>>
}

impl<T> Tree<T>{
    fn new() -> Tree<T>{
        Tree{
            root : None
        }
    }
}

impl<T : Ord> Tree<T>{

    fn insert(& mut self, value : T ) {
        match self.root{
            None => {
                self.root = Node::new(value).into();
            }
            Some(ref mut node) => {
                Tree::<T>::insert_recursive(node, value);
            }
        }
    }

    fn insert_recursive(node : & mut Node<T>, value : T){
        if value > node.value{
            match node.right{
                None => {
                    node.right = Node::new(value).into();
                }
                Some(ref mut n) => {
                    Tree::<T>::insert_recursive(n, value);
                }
            }
        }else if value < node.value{
            match node.left{
                None => {
                    node.left = Node::new(value).into();
                }
                Some(ref mut n) => {
                    Tree::<T>::insert_recursive(n, value);
                }
            }
        }
    }
}

enum TreeIterStatus{
    ReturnedValue,
    Enter,
    ExitLeft,
    ExitRight
}

struct TreeIter<'a, T>{
    tree : & 'a Tree<T>,
    stack : Vec<& 'a Node<T>>,
}

impl<'a, T> TreeIter<'a, T>{
    fn new(tree : & 'a mut  Tree<T>) -> TreeIter<'a, T>{
        TreeIter{
            tree : tree,
            stack : Vec::new()
        }
    }
    fn next_item(& mut self) -> Option<& 'a T>{
        if self.stack.is_empty(){
            match self.tree.root {
                None => { return None; }
                Some(ref node) => {
                    self.stack.push(node);
                }
            }
        }
        while let Some(node) = self.stack.pop(){
            self.stack.push(node);
            match & node.left {
                None => {
                    return Some(& node.value)
                }
                Some(n) => {
                    self.stack.push(& n)
                }
            }
        }
        None
    }
}

impl<'a, T> Iterator for TreeIter<'a, T>{
    type Item = & 'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}

pub struct Stack1 {
    stack: usize
}

impl Stack1 {
    pub fn new() -> Stack1 {
        Stack1 { stack: 1 }
    }

    pub fn push(& mut self, value: bool) -> Result<bool, String>{
        self.stack = self.stack << 1;
        if value{
            self.stack += 1;
        }
        Ok(value)
    }

    pub fn top(& self) -> Result<bool, String> {
        if self.stack == 1 {
            return Err("Empty stack".into())
        }
        Ok((self.stack & 1) == 1)
    }

    pub fn pop(& mut self) -> Result<bool, String> {
        if self.stack == 1 {
            return Err("Empty stack".into())
        }
        let result = (self.stack & 1) == 1;
        self.stack = self.stack >> 1;
        Ok(result)
    }

    pub fn size(& self) -> u32 {
        usize::BITS - usize::leading_zeros(self.stack) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_root_node() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(4);
        tree.insert(6);
        tree.insert(5);
        println!("{:?}",tree);
        let mut tree_iter = TreeIter::new(& mut tree);
        for i in [1..6]{
            println!("{:?}",tree_iter.next())
        }
        assert_eq!(1,1);
    }
}
