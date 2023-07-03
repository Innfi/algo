use chrono::Utc;

pub struct UniqIdGenerator {
  instance_id: i32,
  node_id: i32,
  sequence_id: i32,
}

impl UniqIdGenerator {
  pub fn new(instance_id: i32, node_id: i32) -> Self {
    Self {
      instance_id,
      node_id,
      sequence_id: 0,
    }
  }

  pub fn get_one(&self) -> i64 {
    let now = Utc::now();

    (now.timestamp() << 22) |
    ((self.instance_id << 17) as i64) |
    ((self.node_id << 12) as i64) |
    (self.sequence_id as i64)
  }
}