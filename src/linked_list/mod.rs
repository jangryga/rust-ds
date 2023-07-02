#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

    pub fn reverse(&mut self) {
        let mut prev: Option<Box<Node>> = None;
        let mut curr: Option<Box<Node>> = self.head.take();
    
        while let Some(mut boxed_node) = curr {
            let temp: Option<Box<Node>> = boxed_node.next.take();

            boxed_node.next = prev;
            prev = Some(boxed_node);
            curr = temp;
        }
    
        self.head = prev;
    }
}

#[macro_export]
macro_rules! linked_list {
    () => (
        $crate::linked_list::LinkedList::new();
    );
    ( [ $( $x:expr),* ] ) => {
        {
            let mut temp_list = $crate::linked_list::LinkedList::new();
            $(
                temp_list.add($x);
            )*
            temp_list
        }
    };
    (  $x:expr ) => {
        {
            let mut temp_list = $crate::linked_list::LinkedList::new();
            temp_list.add($x);
            temp_list
        }
    };
}


pub fn merge_sorted_lists(mut ls1: LinkedList, mut ls2: LinkedList) -> LinkedList {
    let mut dummy_head = Box::new(Node::new(0));
    let mut tail = &mut dummy_head;

    while let (Some(mut node1), Some(mut node2)) = (ls1.head.take(), ls2.head.take()) {
        if node1.val > node2.val {
            ls1.head = node1.next.take();
            tail.next = Some(node1);
        } else {
            ls2.head = node2.next.take();
            tail.next = Some(node2);
        }

        tail = tail.next.as_mut().unwrap();
    }

    tail.next = if ls1.head.is_some() { ls1.head } else { ls2.head };
    
    LinkedList { head: dummy_head.next }
}