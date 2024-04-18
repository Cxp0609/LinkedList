use std::fmt;


struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
   
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}


pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
   
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    //  insert an element at the head 
    pub fn insert(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        match self.head.take() {
            Some(old_head) => {
                new_node.next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }

    //  insert an element at the tail 
    pub fn insert_at_tail(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        let mut current = &mut self.head;
        while let Some(ref mut node) = *current {
            current = &mut node.next;
        }
        *current = Some(new_node);
    }

    //  insert an element at a specific index 
    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if index == 0 {
            self.insert(data); // Insert at the head if index is 0
            return;
        }
        let mut current = &mut self.head;
        for _ in 0..(index - 1) { // Move to the node before the target index
            current = &mut current.as_mut().unwrap().next;
        }
        let mut new_node = Box::new(Node::new(data));
        if let Some(ref mut node) = *current {
            new_node.next = node.next.take();
            node.next = Some(new_node);
        } else {
            panic!("Index out of bounds");
        }
    }

    //  delete an element at a specific index 
    pub fn delete(&mut self, index: usize) {
        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }
        match current {
            Some(node) => {
                *current = node.next.take();
            }
            None => {
                panic!("Index out of bounds");
            }
        }
    }

    //  update the value of an element at a specific index 
    pub fn update(&mut self, index: usize, new_data: T) {
        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }
        match current {
            Some(ref mut node) => {
                node.data = new_data;
            }
            None => {
                panic!("Index out of bounds");
            }
        }
    }

    // get the value of an element at a specific index 
    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref()?.next;
        }
        current.as_ref().map(|node| &node.data)
    }
}

// LinkedList to print the list
impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        write!(f, "[")?;
        while let Some(node) = current {
            write!(f, "{}", node.data)?;
            current = &node.next;
            if current.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    // Inserting elements
    list.insert(1);
    list.insert(2);
    list.insert(3);
    println!("List after insertion: {}", list);

    // Inserting at tail
    list.insert_at_tail(4);
    println!("List after inserting at tail: {}", list);

    // Inserting at index
    list.insert_at_index(2, 5);
    println!("List after inserting at index 2: {}", list);

    // Deleting at index
    list.delete(1);
    println!("List after deleting at index 1: {}", list);

    // Updating at index
    list.update(1, 6);
    println!("List after updating at index 1: {}", list);

    // Getting value at index
    let value = list.get(2);
    match value {
        Some(val) => println!("Value at index 2: {}", val),
        None => println!("Index out of bounds"),
    }
}
