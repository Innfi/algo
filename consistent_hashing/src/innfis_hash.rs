pub struct Node {
  pub url: String,
}

pub struct ConsistentHash {
  pub rings: Vec<Node>,
}

impl ConsistentHash {
  pub fn new(mut input_vec: Vec<Node>) -> Self {
    input_vec.sort_by(|a, b| a.url.cmp(&(b.url)));

    Self { rings: input_vec }
  }

  pub fn add_node(&mut self, _new_node: Node) -> Result<usize, &str> {
    
    Ok(0)
  }
}