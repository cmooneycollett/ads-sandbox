use std::cell::RefCell;
use std::iter::FusedIterator;
use std::rc::Rc;

/// An implementation of a doubly-linked list. Not thread-safe.
pub struct LinkedList<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Clone> LinkedList<T> {
    /// Creates an empty LinkedList.
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    /// Pushes the data item to the end of the LinkedList.
    pub fn push(&mut self, data: &T) {
        let new_node: Link<T> = Node::new_link(data);
        // Handle case for empty list
        if self.head.is_none() && self.tail.is_none() {
            self.head = new_node.clone();
            self.tail = new_node;
            return;
        }
        // Update the tail to point at the new node and connect to the old tail
        let old_tail = &self.tail;
        old_tail.as_ref().unwrap().borrow_mut().set_next(&new_node);
        new_node.as_ref().unwrap().borrow_mut().set_prev(old_tail);
        self.tail = new_node;
    }

    /// Pushes the data item to the front of the LinkedList.
    pub fn push_front(&mut self, data: &T) {
        let new_node: Link<T> = Node::new_link(data);
        // Handle case for empty list
        if self.head.is_none() && self.tail.is_none() {
            self.head = new_node.clone();
            self.tail = new_node;
            return;
        }
        // Update the head to point at the new node and connect to the old head
        let old_head = &self.head;
        old_head.as_ref().unwrap().borrow_mut().set_prev(&new_node);
        new_node.as_ref().unwrap().borrow_mut().set_next(old_head);
        self.head = new_node;
    }

    /// Removes the last node from the LinkedList. Returns an Option containing the value from the
    /// removed node, otherwise None.
    pub fn pop(&mut self) -> Option<T> {
        // Handle case for empty list
        if self.head.is_none() && self.tail.is_none() {
            return None;
        }
        // Update the tail to be the second-last node and return value contained in removed node
        let old_tail = self.tail.clone();
        self.tail = old_tail.as_ref().unwrap().borrow().get_prev();
        self.tail.as_ref().unwrap().borrow_mut().set_next(&None);
        let old_data = old_tail.unwrap().borrow().get_data();
        Some(old_data)
    }

    /// Removes the first node from the LinkedList. Returns an Option containing the value from the
    /// removed node, otherwise None.
    pub fn pop_front(&mut self) -> Option<T> {
        // Handle case for empty list
        if self.head.is_none() && self.tail.is_none() {
            return None;
        }
        // Update the head to be the second node and return value contained in removed node
        let old_head = self.head.clone();
        self.head = old_head.as_ref().unwrap().borrow().get_next();
        self.head.as_ref().unwrap().borrow_mut().set_prev(&None);
        let old_data = old_head.unwrap().borrow().get_data();
        Some(old_data)
    }

    /// Returns the number of items contained in the LinkedList.
    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        // Handle case for empty list
        if self.head.is_none() {
            return 0;
        }
        // Proceed through nodes until final node has been counted
        let mut cursor: Link<T> = self.head.clone();
        while cursor.is_some() {
            len += 1;
            cursor = cursor.unwrap().borrow().get_next();
        }
        len
    }

    /// Checks if the LinkedList is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none() && self.tail.is_none()
    }

    /// Creates an iterator over the LinkedList.
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter::new(&self.head)
    }
}

impl<T: Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = LinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Represents a link from one node to another before or after it.
type Link<T> = Option<Rc<RefCell<Box<Node<T>>>>>;

/// A node containing a data item and links to
struct Node<T: Clone> {
    data: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T: Clone> Node<T> {
    /// Creates a new Node containing the given data item. The previous and next node links are set
    /// to None.
    fn new(data: &T) -> Node<T> {
        Node {
            data: data.clone(),
            prev: None,
            next: None,
        }
    }

    /// Updates the previous node.
    fn set_prev(&mut self, other: &Link<T>) {
        self.prev = other.clone();
    }

    /// Updates the next node.
    fn set_next(&mut self, other: &Link<T>) {
        self.next = other.clone();
    }

    /// Gets the previous link from the Node via cloning.
    fn get_prev(&self) -> Link<T> {
        self.prev.clone()
    }

    /// Gets the next link from the Node via cloning.
    fn get_next(&self) -> Link<T> {
        self.next.clone()
    }

    /// Gets the data item contained within the Node via cloning.
    fn get_data(&self) -> T {
        self.data.clone()
    }

    /// Creates a new Link containing the given data item.
    fn new_link(data: &T) -> Link<T> {
        Some(Rc::new(RefCell::new(Box::new(Node::new(data)))))
    }
}

/// Wrapper struct for LinkedList to implement the Iterator trait. Yields cloned values contained in
/// the nodes of the LinkedList.
pub struct LinkedListIter<T: Clone> {
    cursor: Link<T>,
}

impl<T: Clone> LinkedListIter<T> {
    fn new(cursor: &Link<T>) -> LinkedListIter<T> {
        LinkedListIter {
            cursor: cursor.clone(),
        }
    }
}

impl<T: Clone> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if the iterator has been exhausted
        self.cursor.as_ref()?;
        // Get the data to yield and advance the iterator
        let yield_data = self.cursor.as_ref().unwrap().borrow().get_data();
        let next_node = self.cursor.as_ref().unwrap().borrow().get_next();
        self.cursor = next_node;
        Some(yield_data)
    }
}

impl<T: Clone> FusedIterator for LinkedListIter<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back_length() {
        let mut new_list = LinkedList::<i32>::new();
        for i in 0..10 {
            new_list.push(&i);
        }
        assert_eq!(new_list.len(), 10);
    }

    #[test]
    fn test_push_front_length() {
        let mut new_list = LinkedList::<i32>::new();
        for i in 0..10 {
            new_list.push_front(&i);
        }
        assert_eq!(new_list.len(), 10);
    }

    #[test]
    fn test_push_back_values() {
        let mut new_list = LinkedList::<i32>::new();
        let values = (0..10).collect::<Vec<i32>>();
        for i in &values {
            new_list.push(i);
        }
        let values_from_list = new_list.iter().collect::<Vec<i32>>();
        assert_eq!(values, values_from_list);
    }

    #[test]
    fn test_push_front_values() {
        let mut new_list = LinkedList::<i32>::new();
        let values = (0..10).collect::<Vec<i32>>();
        for i in &values {
            new_list.push_front(i)
        }
        let values_from_list = new_list.iter().collect::<Vec<i32>>();
        let values = values.iter().rev().copied().collect::<Vec<i32>>();
        assert_eq!(values, values_from_list);
    }

    #[test]
    fn test_empty_list_length() {
        let new_list = LinkedList::<i32>::new();
        assert_eq!(new_list.len(), 0);
    }

    #[test]
    fn test_list_length_single() {
        let mut new_list = LinkedList::<i32>::new();
        new_list.push(&1);
        assert_eq!(new_list.len(), 1);
    }

    #[test]
    fn test_list_str_push_back() {
        let mut new_list = LinkedList::<&str>::new();
        let strings = ["10", "20", "30", "40", "50"].to_vec();
        for s in &strings {
            new_list.push(s);
        }
        let strings_from_list = new_list.iter().collect::<Vec<&str>>();
        assert_eq!(strings, strings_from_list);
    }

    #[test]
    fn test_iter_values() {
        let mut new_list = LinkedList::<i32>::new();
        let values = (0..10).collect::<Vec<i32>>();
        for i in &values {
            new_list.push(i);
        }
        let mut values_from_list: Vec<i32> = vec![];
        for i in new_list {
            values_from_list.push(i);
        }
        assert_eq!(values, values_from_list);
    }
}
