macro_rules! print_linked_list {
    ($list:expr) => {
        {
            let mut current = &$list.head;
            print!("Linked List: ");
            while let Some(node) = current {
                print!("{:?} -> ", node.data);
                current = &node.next;
            }
            println!("None");
        }
    };
}

fn main() {
    let mut list = LinkedList::new();

    list.insert(5);
    list.insert(10);
    list.insert(15);
    list.insert(20);

    println!("Initial list:");
    print_linked_list!(list); // Output: 20 -> 15 -> 10 -> 5-> None

    list.insert_at_tail(25);

    println!("List after inserting at tail:");
    print_linked_list!(list); // Output: 20 -> 15 -> 10 -> 5 -> 25 -> None

    list.insert_at_index(30, 2);

    println!("List after inserting at index 2:");
    print_linked_list!(list); // Output: 20 -> 15 -> 30 -> 10 -> 5 -> 25 -> None

    list.delete_at_index(3);

    println!("List after deleting at index 3:");
    print_linked_list!(list); // Output: 20 -> 15 -> 25 -> 10 -> 5 -> None

    list.update(2, 35);

    println!("List after updating at index 2:");
    print_linked_list!(list); // Output: 20 -> 15 -> 35 -> 5 -> 25 -> None

    println!("Element at index 3: {:?}", list.get(3)); // Output: Some(5)
}

use std::fmt;

/// Define a Node struct containing data and a reference to the next node
struct Node<J> {
    data: J,
    next: Option<Box<Node<J>>>,
}

impl<J> Node<J> {
    /// Constructor for Node
    fn new(data: J) -> Self {
        Node { data, next: None }
    }
}

/// Define a structure for a linked list.
pub struct LinkedList<J> {
    head: Option<Box<Node<J>>>,
    size: usize,
}

impl<J> LinkedList<J> {
    /// Constructor for LinkedList
    pub fn new() -> Self {
        LinkedList { head: None, size: 0 }
    }

     /// Insert data at the start of the list.
        pub fn insert(&mut self, data: J) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.size += 1;
    }

     /// Insert data at the tail of the list.
        pub fn insert_at_tail(&mut self, data: J) {
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        *current = Some(Box::new(Node::new(data)));
        self.size += 1;
    }

    /// Insert data at a particular index within the list.

    pub fn insert_at_index(&mut self, data: J, index: usize) {
        if index > self.size {
            panic!("Index out of bounds");
        }
        if index == 0 {
            self.insert(data);
            return;
        }
        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            current = &mut current.as_mut().unwrap().next;
        }
        let mut new_node = Box::new(Node::new(data));
        new_node.next = current.as_mut().unwrap().next.take();
        current.as_mut().unwrap().next = Some(new_node);
        self.size += 1;
    }

    /// Need to delete node at a specific index
    pub fn delete_at_index(&mut self, index: usize) {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }
        *current = current.as_mut().unwrap().next.take();
        self.size -= 1;
    }

    /// Update data at a specific index
    pub fn update(&mut self, index: usize, data: J) {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        let mut current = &mut self.head;
        for _ in 0..index {
            current = &mut current.as_mut().unwrap().next;
        }
        current.as_mut().unwrap().data = data;
    }

    /// Retrieve the data at a specific index.
    pub fn get(&self, index: usize) -> Option<&J> {
        if index >= self.size {
            return None;
        }
        let mut current = &self.head;
        for _ in 0..index {
            current = &current.as_ref().unwrap().next;
        }
        Some(&current.as_ref().unwrap().data)
    }
}

