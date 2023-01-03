#[cfg(test)]
mod tests_double_link {
  use std::{rc::Rc, borrow::BorrowMut};
  use std::cell::RefMut;

  pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
  }

  type Link<T> = Option<Rc<Node<T>>>;

  struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
  }

  impl<T> List<T> {
    pub fn new() -> Self {
      List { head: None, tail: None }
    }

    pub fn add(&mut self, elem: T) {
      if self.head.is_none() {
        self.head = Some(Rc::new(Node {
          elem,
          prev: None,
          next: None,
        }));

        return
      }

      let new_tail = Rc::new(Node {
        elem,
        prev: self.head.clone(),
        next: self.head.clone(),
      });

      // TODO: link next and prev between nodes

      let b1 = self.head.borrow_mut();
      let mut test_ref = RefMut::map(b1, |t| &mut t);

      // let mut der = self.head.borrow_mut().as_ref().unwrap();
      // der.next = Some(new_tail);
    }

    pub fn to_head(&self) -> Option<&T> {
      self.head.as_ref().map(|node| &node.elem)
    }

    pub fn to_tail(&self) -> Option<&T>  {
      self.tail.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
      Iter { next: self.head.as_deref(), prev: None }
    }
  }

  pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
    prev: Option<&'a Node<T>>,
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

  impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
      self.prev.map(|node| {
        self.prev = node.prev.as_deref();
  
        &node.elem
      })
    }
  }

  #[test]
  fn test_head_default() {
    let instance = List::<i32>::new();

    assert_eq!(instance.to_head(), None);
  }

  #[test]
  fn test_first_node() {
    let mut instance = List::<i32>::new();

    instance.add(1);

    assert_eq!(instance.to_head(), Some(&1));
  }

  #[test]
  fn test_second_node_with_iterator() {
    let mut instance = List::<i32>::new();

    instance.add(1);
    instance.add(2);

    let mut iter = instance.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
  }

  #[test]
  fn test_iterator_rev() {
    let input = vec![1, 2, 3, 4];

    let mut iter = input.iter().rev();

    assert_eq!(iter.next(), Some(&4));
  }
}