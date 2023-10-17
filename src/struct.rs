use std::ptr::null_mut;
use std::fmt::{Display, Formatter, Result, Debug};

#[derive(Debug)]
struct Node{
  left: *mut Node,
  right: *mut Node,
  value: i8
}

impl Display for Node {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    if let Err(err) = write!(f, "Node {:p} {{ left: {:?}, right: {:?}, value: {:?} }}", self, self.left, self.right, self.value){
      return Err(err);
    }
    Ok(())
  }
}

fn main() {
  let mut nodes: Vec<&Node> = vec![];
  const NULL_NODE: Node = Node {
    left: null_mut(),
    right: null_mut(),
    value: 0
  };
  nodes.push(&NULL_NODE);

  let mut head: Node = Node{
    left: null_mut(),
    right: null_mut(),
    value: 0,
  };

  head.left = &mut Node{
    left: null_mut(),
    right: null_mut(),
    value: -1,
  } as *mut Node;

  head.right = &mut Node{
    left: null_mut(),
    right: null_mut(),
    value: 1,
  } as *mut Node;

  nodes.push(&head);
  nodes.push(unsafe {&*head.left});
  nodes.push(unsafe {&*head.right});
  for node in nodes {
    println!("Node: {}", node);
  }
}