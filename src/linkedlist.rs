#[derive(Clone)]
pub struct Node<T:Clone> {
    pub value:T,
    pub next:Option<Box<Node<T>>>
}

impl<T:Clone> Node<T> {
    pub fn forward_node(&mut self){
        if let Some(node) = self.next.as_mut() {
            self.next = node.next.take();
        }
    }

    pub fn insert_value(&mut self, value:T){
        let new_node = Node {
            value, next:self.next.take()
        };
        self.next = Some(Box::new(new_node));
    }
}

pub struct LinkedList<T:Clone> {
    head:Option<Box<Node<T>>>,
    size:usize,
    current_index:usize
}

impl<T:Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, size:0, current_index:0 }
    }

    pub fn push_end(&mut self, value:T){
        if self.head.is_none() {
            self.head = Some( Box::new( Node{ value:value, next:None } ) );
            self.size += 1;
        }
        else {
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some( Box::new( Node{ value:value.clone(), next:None } )) ;
                }
                current = &mut node.next;
            }
            self.size += 1;
        }
    }

    pub fn push_front(&mut self, value:T){
        let new_node = Node {
            value,
            next:self.head.take()
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    pub fn extend_end(&mut self, values:&[T]){
        for value in values {
            self.push_end(value.clone());
            self.size += 1;
        }
    }

    pub fn extend_front(&mut self, values:&[T]){
        for value in values.iter().rev() {
            self.push_front(value.clone());
            self.size += 1;
        }
    }

    pub fn clear(&mut self){
        self.size = 0;
        self.head = None;
    }
}