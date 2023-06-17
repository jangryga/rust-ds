#[derive(Debug)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn new(v: i32) -> Self {
        Node {
            next: None,
            val: v
        }
    }
}

#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None
        }
    }

    pub fn add(&mut self, val: i32) {
        let new_node = Box::new(Node::new(val));

        let mut curr = &mut self.head;

        while let Some(ref mut node) = *curr {
            curr = &mut node.next;
        }

        *curr = Some(new_node);
    }
}