#[cfg(test)]
mod tests {
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

  #[test]
  fn node_without_option() {
    // how to remove Option<>?
  }
// 	use linked_container::linked_list::*;

// 	#[test]
// 	fn test_insert_initial() {
// 		let mut instance = SingleLinkedList::new();

// 		let input = String::from("hello");
// 		let _ = instance.insert(input);

// 		let root = instance.get_root().unwrap();

// 		assert_eq!(root.name.as_str(), "hello");
// 	}

// 	#[test]
// 	fn test_next_node() {
// 		let mut instance = SingleLinkedList::new();

// 		let _ = instance.insert(String::from("hello"));
// 		let _ = instance.insert(String::from("world"));

// 		let root = instance.get_root().unwrap();

// 		assert_eq!(root.name.as_str(), "hello");
// 		assert_eq!(root.next.as_ref().unwrap().name.as_str(), "world");
// 	}
}
