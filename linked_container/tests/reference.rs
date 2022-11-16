
// https://rust-unofficial.github.io/too-many-lists
#[cfg(test)]
mod tests_reference1 {
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

#[cfg(test)]
mod tests_reference2 {
  pub struct List<T> {
    head: Link<T>,
  }

  type Link<T> = Option<Box<Node<T>>>;

  struct Node<T> {
    elem: T,
    next: Link<T>,
  }

  impl<T> List<T> {
    pub fn new() -> Self {
      List { head: None }
    }

    pub fn push(&mut self, elem: T) {
      let new_node = Box::new(Node {
        elem: elem,
        next: self.head.take(),
      });

      self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
      self.head.take().map(|node| {
        self.head = node.next;
        node.elem
      })
    }

    pub fn into_iter(self) -> IntoIter<T> {
      IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
      Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
      IterMut { next: self.head.as_deref_mut() }
    }
  }

  impl<T> Drop for List<T> {
    fn drop(&mut self) {
      let mut cur_link = self.head.take();
      while let Some(mut boxed_node) = cur_link {
        cur_link = boxed_node.next.take();
      }
    }
  }

  pub struct IntoIter<T>(List<T>);

  impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
      self.0.pop()
    }
  }

  pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,    
  }

  impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      self.next.map(|node| {
        self.next = node.next.as_deref();
        &node.elem
      })
    }
  }

  pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,    
  }

  impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
      self.next.take().map(|node| {
        self.next = node.next.as_deref_mut();
        &mut node.elem
      })
    }
  }

  #[test]
  fn test_push_pop() {
    let mut list = List::new();
    list.push(String::from("begin"));
    list.push(String::from("middle"));
    list.push(String::from("end"));

    assert_eq!(list.pop(), Some(String::from("end")));
    list.pop();
    list.pop();
    assert_eq!(list.pop(), None);
  }

  #[test]
  fn test_into_iter() {
    let mut list = List::new();
    list.push(String::from("begin"));
    list.push(String::from("middle"));
    list.push(String::from("end"));

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(String::from("end")));

    // illegal as list is moved
    // assert_eq!(list.pop(), Some(String::from("middle")));
  }

  #[test]
  fn test_iter() {
    let mut list = List::new();
    list.push(String::from("begin"));
    list.push(String::from("middle"));
    list.push(String::from("end"));

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&String::from("end")));

    // list is live and well
    assert_eq!(list.pop(), Some(String::from("end")));
  }
}

mod tests_reference3 {
  use std::rc::Rc;

  pub struct List<T> {
    head: Link<T>
  }

  type Link<T> = Option<Rc<Node<T>>>;

  struct Node<T> {
    elem: T,
    next: Link<T>
  }

  impl<T> List<T> {
    pub fn new() -> Self {
      List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
      List {
        head: Some(Rc::new(
          Node {
            elem: elem,
            next: self.head.clone(),
          }
        ))
      }
    }

    pub fn tail(&self) -> List<T> {
      List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
      self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
      Iter { next: self.head.as_deref() }
    }
  }

  impl<T> Drop for List<T> {
    fn drop(&mut self) {
      let mut head = self.head.take();
      while let Some(node) = head {
        if let Ok(mut node) = Rc::try_unwrap(node) {
          head = node.next.take();
        } else {
          break;
        }
      }
    }
  }

  pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
  }

  impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
      self.next.map(|node| {
        self.next = node.next.as_deref();

        &node.elem
      })
    }
  }

  #[test]
  fn initial_empty_head() {
    let list = List::<i32>::new();

    assert_eq!(list.head(), None);
  }

  #[test]
  fn head_and_tail() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    assert_eq!(list.head(), Some(&3));

    let next_list = list.tail();
    assert_eq!(next_list.head(), Some(&2));
  }

  #[test]
  fn iterators() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    let mut iter = list.iter();

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
  }
}

mod tests_reference4 {
  use std::rc::Rc;
  use std::cell::{Ref, RefMut, RefCell};

  pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
  }

  type Link<T> = Option<Rc<RefCell<Node<T>>>>;

  struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
  }

  impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
      Rc::new(RefCell::new(Node {
        elem,
        prev: None,
        next: None,
      }))
    }
  }

  impl<T> List<T> {
    pub fn new() -> Self {
      List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
      let new_instance = Node::new(elem);

      match self.head.take() {
        None => {
          self.head = Some(new_instance.clone());
          self.tail = Some(new_instance);
        },
        Some(old_head) => {

        }
      }
    }
  }
}