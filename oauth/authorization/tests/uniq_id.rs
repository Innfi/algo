use chrono::Utc;

#[test]
fn initial_test() {
  assert_eq!(1, 1);
}

#[test]
fn test_datetime() {
  let now = Utc::now();

  let as_timestamp = now.timestamp();
  print!("time: {}\n", as_timestamp);

  let instance_id: i32 = 1234;
  let node_id: i32 = 4567;
  let sequence_id: i32 = 11;

  let snowflake_test: i64 = (as_timestamp << 22) |
    ((instance_id << 17) as i64) |
    ((node_id << 12) as i64) |
    (sequence_id as i64);

  assert_eq!(as_timestamp > 0, true);

  print!("snowflake_test: {}\n", snowflake_test);

  assert_eq!(snowflake_test > 0, true);
}