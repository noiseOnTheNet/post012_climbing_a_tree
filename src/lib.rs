#[derive(Debug)]
pub struct BTree0<T>{
    pub content : T,
    pub right : Option<Box<BTree0<T>>>,
    pub left : Option<Box<BTree0<T>>>
}

impl Iterator for BTree0<T> {
    type Item<T> = BTree<T>;
    fn next(self){
        return None;
    }

}
