use std::collections::HashMap;

struct Node {
  url: String,
}

pub struct ConsistentHash {
  rings: HashMap<String, Node>,
}

impl ConsistentHash {
  pub fn new() -> Self {
    Self { rings: HashMap::new() }
  }

  pub fn initial_runner(&self) -> String {
    String::from("start from here")
  }

  pub fn add_node(&mut self, new_server: Node) {    
    self.rings.insert(self.to_hash(), new_server);
  }

  pub fn remove_node(&mut self, target_server: Node) {
    let key = self.to_hash();
    self.rings.remove_entry(&key);
  }

  fn to_hash(&self) -> String {
    //TODO: hash
    String::from("test")
  }
}
