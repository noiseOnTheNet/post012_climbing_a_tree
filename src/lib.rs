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
            value,
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
    Enter,
    LeftCompleted,
    ValueYield,
    Completed
}

pub struct TreeIter<'a, T> {
    stack : Vec<(NodeStatus, & 'a Node<T>)>
}

impl<'a, T> TreeIter<'a, T>{
    fn new(tree : & 'a Tree<T>) -> TreeIter<'a, T>{
        match tree.root {
            None => {
                TreeIter{
                    stack : Vec::new()
                }
            }
            Some(ref node) => {
                TreeIter{
                    stack: vec![(NodeStatus::Enter, & node)]
                }
            }
        }
    }
    fn next_item(& mut self) -> Option<& 'a T>{
        while let Some((address,node)) = self.stack.pop(){
            match address {
                NodeStatus::Enter => {
                    match node.left{
                        None => {
                            self.stack.push((NodeStatus::LeftCompleted, node));
                        },
                        Some(ref left) => {
                            self.stack.push((NodeStatus::LeftCompleted, node));
                            self.stack.push((NodeStatus::Enter, left));
                        }
                    }
                },
                NodeStatus::LeftCompleted => {
                    self.stack.push((NodeStatus::ValueYield, node));
                    return Some(& node.value);
                },
                NodeStatus::ValueYield => {
                    match node.right{
                        None => {
                            self.stack.push((NodeStatus::Completed, node));
                        },
                        Some(ref right) => {
                            self.stack.push((NodeStatus::Completed, node));
                            self.stack.push((NodeStatus::Enter, right));
                        }
                    }
                },
                NodeStatus::Completed => {
                },
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

impl<'a , T> IntoIterator for & 'a Tree<T>{
     type Item = & 'a T;
     type IntoIter = TreeIter<'a, T>;
     fn into_iter(self) -> Self::IntoIter {
         TreeIter::new(self)
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
        let mut tree : Tree<i64>= Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(4);
        tree.insert(6);
        tree.insert(5);
        println!("{:?}",tree);
        let result : Vec<i64> = tree.into_iter().map(|x| (*x).clone()).collect();
        assert_eq!(result,vec![4,5,6,8,10]);
    }
}
