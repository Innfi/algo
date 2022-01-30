use std::collections::HashMap;

pub struct Node {
  pub url: String,
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

  pub fn add_node(&mut self, new_server: Node) -> Option<bool> {    
    let key_string = self.to_hash();

    if self.rings.contains_key(&key_string) {
      None
    } else {
      self.rings.insert(key_string, new_server);
      Some(true)
    }
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
