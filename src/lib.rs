use std::fmt::Debug;

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

#[derive(Debug, Copy, Clone)]
enum NodeStatus{
    A,
    B,
    C,
    D
}

struct TreeIter<'a, T: Debug> {
    stack : Vec<(NodeStatus, & 'a Node<T>)>
}

impl<'a, T : Debug> TreeIter<'a, T>{
    fn new(tree : & 'a mut  Tree<T>) -> TreeIter<'a, T>{
        match tree.root {
            None => {
                TreeIter{
                    stack : Vec::new()
                }
            }
            Some(ref node) => {
                TreeIter{
                    stack: vec![(NodeStatus::A, & node)]
                }
            }
        }
    }
    fn next_item(& mut self) -> Option<& 'a T>{
        while let Some((address,node)) = self.stack.pop(){
            println!("entering {:?} :: {:?} :: stack {}",address,node.value, self.stack.len());
            match address {
                NodeStatus::A => {
                    match node.left{
                        None => {
                            self.stack.push((NodeStatus::B, node));
                        },
                        Some(ref left) => {
                            self.stack.push((NodeStatus::B, node));
                            self.stack.push((NodeStatus::A, left));
                            println!("A adding {:?} :: {:?} :: stack {}","A",left.value, self.stack.len());
                        }
                    }
                },
                NodeStatus::B => {
                    self.stack.push((NodeStatus::C, node));
                    return Some(& node.value);
                },
                NodeStatus::C => {
                    match node.right{
                        None => {
                            self.stack.push((NodeStatus::D, node));
                        },
                        Some(ref right) => {
                            self.stack.push((NodeStatus::D, node));
                            self.stack.push((NodeStatus::A, right));
                            println!("C adding {:?} :: {:?} :: stack {}","A",right.value, self.stack.len());
                        }
                    }
                },
                NodeStatus::D => {
                    println!("D exiting :: {:?} :: stack {}",node.value, self.stack.len());
                },
            }
        }
        None
    }
}

impl<'a, T : Debug> Iterator for TreeIter<'a, T>{
    type Item = & 'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}

// impl<'a, T> IntoIterator for Tree<T>{
//     type Item = & 'a T;
//     type IntoIter = TreeIter<'a , T>;
//     fn into_iter(mut self) -> Self::IntoIter {
//         TreeIter::new(& mut self)
//     }
// }

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
        for i in 1..12{
            println!("{} ==> {:?}",i,tree_iter.next())
        }
        assert_eq!(0,1);
    }
}
