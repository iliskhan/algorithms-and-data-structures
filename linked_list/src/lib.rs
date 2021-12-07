#[derive(Clone)]
struct Node<T> {
    data: T,
    next: Box<Option<Node<T>>>,
}

struct List<T> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
}

#[cfg(test)]
mod tests {
    use crate::{List, Node};

    #[test]
    fn test_node() {
        let node = Node {
            data: "some node data",
            next: Box::new(None),
        };
        assert_eq!(node.data, "some node data");
        assert!(node.next.is_none());
    }

    #[test]
    fn test_list() {
        let second_node = Node {
            data: 211,
            next: Box::new(None),
        };
        let first_node = Node {
            data: 112,
            next: Box::new(Some(second_node.clone())),
        };
        let list = List {
            head: Some(first_node.clone()),
            tail: Some(second_node.clone()),
        };
        assert!(list.head.is_some());
        if let Some(head) = list.head {
            assert_eq!(head.data, first_node.data);
        }
        if let Some(tail) = list.tail {
            assert_eq!(tail.data, second_node.data);
        }
    }
}
