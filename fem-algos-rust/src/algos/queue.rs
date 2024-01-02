#[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl List<usize> {
    //the ideal scenario is a reference return
    fn peek(self) -> usize {
        self.head.unwrap().elem
    }
    fn enqueue(&mut self, value: usize) {
        let new_node = Node {
            elem: value,
            next: None,
        };
        match self.tail.as_mut() {
            Some(mut tail) => {
                tail.next = Some(Box::new(new_node.clone()));
                tail = &mut Box::new(new_node);
            }
            None => self.tail = Some(Box::new(new_node)),
        }
    }

    fn deque(&mut self) -> Option<usize> {
        match &self.head {
            Some(node) => {
                let dequeued_opt_clone = node.clone();
                self.head = dequeued_opt_clone.next;
                Some(dequeued_opt_clone.elem)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_queue() {
        let mut queue = List {
            head: None,
            tail: None,
        };
        List::enqueue(&mut queue, 5);
        List::enqueue(&mut queue, 7);
        List::enqueue(&mut queue, 9);

        assert_eq!(List::deque(&mut queue), Some(5));
        assert_eq!(List::deque(&mut queue), Some(7));
        // assert_eq!(List::length).toEqual(2);

        List::enqueue(&mut queue, 11);

        assert_eq!(List::deque(&mut queue), Some(7));
        assert_eq!(List::deque(&mut queue), Some(9));
        // assert_eq!(List::peek(&mut queue), 11);
        assert_eq!(List::deque(&mut queue), Some(11));
        // assert_eq!(List::deque(&mut queue), None);
        // assert_eq!(List::length, 0);
        List::enqueue(&mut queue, 69);
        // assert_eq!(List::peek(), 69);
        // assert_eq!(List::length, 1);
    }
}
