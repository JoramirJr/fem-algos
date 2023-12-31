#[derive(Clone)]
struct Node<T> {
    elem: T,
    next: Box<Node<T>>,
}

struct List<T> {
    head: Node<T>,
    tail: Node<T>,
}

impl List<usize> {
    //the ideal scenario is a reference return
    fn peek(self) -> Node<usize> {
        self.head
    }
    fn enqueue(mut self, elem: Node<usize>) {
        self.tail.next = Box::new(elem);
        self.tail = elem;
    }

    fn deque(&self) -> Node<usize> {
        let boxed_old_head_next: &Node<_> = &self.head.next;
        self.head = *boxed_old_head_next;
        self.head
    }
}
