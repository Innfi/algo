use rand::prelude::*;

use token_bucket::bucket_impl::RequestToken;

#[test]
fn test_initial() {
  let initial_token = RequestToken {
    id: String::from("hello")
  };

  assert_eq!(initial_token.id.as_str(), "hello");
}

#[test]
fn test_random() {
  let mut rng = rand::thread_rng();

  let mut prev: u32 = 0;

  for _ in 1..=100 {
    let test_float: u32 = rng.gen();
    assert_eq!(prev != test_float, true);

    prev = test_float;
  }
}