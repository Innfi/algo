#[cfg(test)]
mod tests_option {
  struct TestNode {
    pub data: String,
    pub next: Option<Box<TestNode>>,
  }
  
  impl TestNode {
    pub fn new(input: String) -> Self {
      Self {
        data: input,
        next: None
      }
    }

    pub fn new_box(input: String) -> Box<Self> {
      Box::new(Self {
        data: input,
        next: None,
      })
    }
  }

  #[test] 
  fn test_new() {
    let first_node = TestNode::new(String::from("test"));

    assert_eq!(first_node.data, String::from("test"));
    assert_eq!(first_node.next.is_none(), true);
  }

  #[test]
  fn test_box_new() {
    let first_node = TestNode::new_box(String::from("test"));

    assert_eq!(first_node.data, String::from("test"));
    assert_eq!(first_node.next.is_none(), true);
  }

  #[test]
  fn link_node() {
    let mut root_node = TestNode::new_box(String::from("first"));
    let new_node = TestNode::new_box(String::from("next"));

    root_node.next = Some(new_node);

    assert_eq!(root_node.next.is_none(), false);

    let next_node = root_node.next.unwrap();
    assert_eq!(next_node.data, String::from("next"));
    assert_eq!(next_node.next.is_none(), true);
  }

  #[test]
  fn traverse_link() {
    let mut root_node = TestNode::new_box(String::from("first"));
    let mut node1 = TestNode::new_box(String::from("node1"));
    let mut node2 = TestNode::new_box(String::from("node2"));
    let node3 = TestNode::new_box(String::from("node3"));
    
    node2.next = Some(node3);
    node1.next = Some(node2);
    root_node.next = Some(node1);

    // what the hell...
    let leaf_node = root_node.next.unwrap().next.unwrap().next.unwrap();

    assert_eq!(leaf_node.data, String::from("node3"));
    assert_eq!(leaf_node.next.is_none(), true);
  }

}
