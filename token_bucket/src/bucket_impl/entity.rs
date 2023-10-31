use serde::{Deserialize, Serialize};

pub const QUEUE_LEN: usize = 100;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestToken {
  pub id: String,
}