use chrono::{Duration, Utc};

#[test]
fn test_add_time() {
  let now = Utc::now();

  let mut as_timestamp: i64 = now.timestamp();
  as_timestamp += 1000;

  let future_now = now + Duration::seconds(1000);

  assert_eq!(future_now.timestamp() == as_timestamp, true);
}
