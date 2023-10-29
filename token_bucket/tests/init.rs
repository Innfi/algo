use token_bucket::bucket_impl::RequestToken;

#[test]
fn test_initial() {
  let initial_token = RequestToken {
    id: String::from("hello")
  };

  assert_eq!(initial_token.id.as_str(), "hello");
}