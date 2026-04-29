
struct Node {
    next: Option<Box<Node>>,
    data: i32,
}
struct LinkedList {
    head: Option<Box<Node>>,
}
impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            next: self.head.take(),
            data: value,
        });
        self.head = Some(new_node)
    }
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
    fn push_right(&mut self, value: i32) {
        let mut current = &mut self.head;
        let new_node = Box::new(Node {
            next: None,
            data: value,
        });
        loop {
            match current {
                Some(node) => current = &mut node.next,
                None => {
                    *current = Some(new_node);
                    break;
                }
            }
        }
    }
    fn pop_right(&mut self) -> Option<i32> {
        // Case 1: List is empty
        if self.head.is_none() {
            return None;
        }

        // Case 2: List has only one element
        if self.head.as_ref()?.next.is_none() {
            return self.head.take().map(|node| node.data);
        }

        // Case 3: List has 2+ elements.
        // We need to find the node whose next node's next is None.
        let mut current = self.head.as_mut()?;

        while current.next.as_ref()?.next.is_some() {
            current = current.next.as_mut()?;
        }

        // current is now the second-to-last node.
        // Take the last node, leaving None in its place.
        current.next.take().map(|node| node.data)
    }

    fn print(&self) {}
}
