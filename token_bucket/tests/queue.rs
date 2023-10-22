use concurrent_queue::ConcurrentQueue;

#[test]
fn test_initial() {
  let q = ConcurrentQueue::bounded(5);

  q.push(1).unwrap();
  q.push(2).unwrap();

  assert_eq!(q.pop(), Ok(1));
}

#[test]
fn test_len() {
  let q = ConcurrentQueue::bounded(5);

  q.push(1).unwrap();
  q.push(2).unwrap();

  assert_eq!(q.len(), 2);

  q.pop().unwrap();

  assert_eq!(q.len(), 1);
}

#[test]
fn test_limit() {
  let q = ConcurrentQueue::bounded(3);

  q.push(1).unwrap();
  q.push(2).unwrap();
  q.push(3).unwrap();

  assert_eq!(q.is_full(), true);

  let result = q.push(4);
  assert_eq!(result.is_err(), true);
}