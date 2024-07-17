use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            children: vec![],
        }))
    }

    fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        parent.borrow_mut().children.push(child);
    }

    fn print_tree(node: &Rc<RefCell<Node>>, depth: usize) {
        let indent = " ".repeat(depth * 2);
        println!("{}{}", indent, node.borrow().value);
        for child in node.borrow().children.iter() {
            Node::print_tree(child, depth + 1);
        }
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::add_child(&root, child1.clone());
    Node::add_child(&root, child2.clone());
    Node::add_child(&child1, Node::new(4));
    Node::add_child(&child1, Node::new(5));
    Node::add_child(&child2, Node::new(6));

    Node::print_tree(&root, 0);
}
