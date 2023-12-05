use concurrent_queue::ConcurrentQueue;

#[derive(Debug)]
pub struct ReqCountUnit {
  id: usize,
  count: usize,
}

#[test]
fn test_concurrent_queue_iter() {
  let queue: ConcurrentQueue<ReqCountUnit> = ConcurrentQueue::bounded(100);

  for n in 0..10 {
    queue.push(ReqCountUnit {
      id: n,
      count: 1,
    }).expect("push error");
  }

  let mut iter = queue.try_iter();
  let sum = iter.by_ref().map(|x| {
    println!("id: {}", x.id);
    x.count
  }).reduce(|a, b| a+b);
  assert_eq!(sum, Some(10));
}

#[test]
fn test_mutate_and_get_sum() {
  let queue: ConcurrentQueue<ReqCountUnit> = ConcurrentQueue::bounded(100);

  for n in 0..10 {
    queue.push(ReqCountUnit {
      id: n,
      count: 1,
    }).expect("push error");
  }

  queue.pop().expect("pop error");
  queue.push(ReqCountUnit { id: 20, count: 3 }).expect("push error again");

  assert_eq!(queue.len(), 10);
  let sum = queue.try_iter().by_ref().map(|x| {
    println!("id: {}", x.id);
    x.count
  }).reduce(|a, b| a+b);
  assert_eq!(sum, Some(12));
}