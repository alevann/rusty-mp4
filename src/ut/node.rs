use std::fmt::{self, Debug};

pub struct Node<T>
{
    nodes: Vec<Node<T>>,
    data: Option<T>
}

impl <T> Node<T> {
    pub fn new() -> Node<T> {
        Node {
            nodes: Vec::new(),
            data: None
        }
    }

    pub fn from(data: T) -> Node<T> {
        Node {
            nodes: Vec::new(),
            data: Some(data)
        }
    }

    pub fn add(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub fn add_if_some(mut self, node: Option<Node<T>>) -> Self {
        if let Some(node) = node {
            self.add(node)
        }
        self
    }
}

impl <T> Debug for Node<T> where T : Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let deb: Box<dyn Debug> 
        = if let Some(data) = &self.data { 
            Box::new(data)
        } else { 
            Box::new(String::from("None"))
        };

        f.debug_struct("Node")
        .field("data", &deb)
         .field("nodes", &self.nodes)
         .finish()
    }
}
