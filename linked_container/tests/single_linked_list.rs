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

// https://rust-unofficial.github.io/too-many-lists/first-final.html
#[cfg(test)]
mod tests_reference {
  use std::mem;

  pub struct List {
    head: Link,
  }

  enum Link {
    Empty,
    More(Box<Node>),
  }

  struct Node {
    elem: i32,
    next: Link,
  }

  impl List {
    pub fn new() -> Self {
      List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
      let new_node = Box::new(Node {
        elem: elem,
        next: mem::replace(&mut self.head, Link::Empty),
      });

      self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
      match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => None,
        Link::More(node) => {
          self.head = node.next;
          Some(node.elem)
        }
      }
    }
  }

  impl Drop for List {
    fn drop(&mut self) {
      let mut cur_link = mem::replace(&mut self.head, Link::Empty);

      while let Link::More(mut boxed_node) = cur_link {
        cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
      }
    }
  }

  #[test]
  fn test_empty_list() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);
  }

  #[test]
  fn test_push_pop() {
    let mut list = List::new();

    list.push(5);
    assert_eq!(list.pop().unwrap(), 5);
    assert_eq!(list.pop(), None);
  }
}