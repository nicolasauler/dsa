#![allow(dead_code)]
#![allow(unused_variables)]

struct LinkedList {
    head: Option<Box<Node>>,
}

#[derive(Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

// a -> b -> c -> d -> None
// prev = None
// current = a
// next = b
// a.next = None
// prev = a
// current = b
// a -> None
// next = c
// b.next = a
// prev = b
// current = c
// b -> a -> None
// next = d
// c.next = b
// prev = c
// current = d
// c -> b -> a -> None
// next = None
// d.next = c
// prev = d
// current = None
// d -> c -> b -> a -> None
//
// linked_list.head = d
fn reverse_linked_list(linked_list: &mut LinkedList) {
    let mut prev = None;
    let mut current = linked_list.head.take();
    while let Some(mut current_node) = current {
        let next = current_node.next.take();
        current_node.next = prev.take();
        prev = Some(current_node);
        current = next;
    }
    linked_list.head = prev;
}

fn main() {
    let mut linked_list = LinkedList {
        head: Some(Box::new(Node {
            value: 1,
            next: Some(Box::new(Node {
                value: 2,
                next: Some(Box::new(Node {
                    value: 3,
                    next: None,
                })),
            })),
        })),
    };
    reverse_linked_list(&mut linked_list);
}
