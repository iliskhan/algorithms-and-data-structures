use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

fn create_data_pointer<T>(data: T) -> Pointer<T> {
    Some(Rc::new(RefCell::new(Node::new(data))))
}

#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Pointer<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

struct List<T> {
    head: Pointer<T>,
    tail: Pointer<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn add_node_at_end(&mut self, data: T) {
        let data_pointer = create_data_pointer(data);
        if self.head.is_none() {
            self.head = data_pointer.clone();
        } else {
            let tail = self.tail.take();
            if let Some(old_tail) = tail {
                old_tail.borrow_mut().next = data_pointer.clone();
            }
        }
        self.tail = data_pointer;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Ref;
    use std::ops::Deref;

    use crate::{List, Node};

    #[test]
    fn test_node() {
        let node = Node::new("some node data");
        assert_eq!(node.data, "some node data");
        assert!(node.next.is_none());
    }

    #[test]
    fn test_list() {
        let mut list = List::new();
        list.add_node_at_end(1);
        list.add_node_at_end(2);
        list.add_node_at_end(3);
        assert!(list.head.is_some());
        if let Some(head) = list.head.take() {
            let head_node = head.as_ref().borrow();
            assert_eq!(head_node.data, 1);
            assert_eq!(head_node.next.as_ref().unwrap().borrow().data, 2);
        }

        if let Some(tail) = list.tail {
            assert_eq!(tail.borrow_mut().data, 3);
        }
    }
}
