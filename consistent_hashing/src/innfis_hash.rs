use std::collections::HashMap;

pub struct Node {
  pub url: String,
  pub keys: Vec<String>,
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
    let key_string = self.to_hashkey(&new_server);

    if self.rings.contains_key(&key_string) {
      None
    } else {
      self.rings.insert(key_string, new_server);
      Some(true)
    }
  }

  pub fn remove_node(&mut self, target_server: Node) {
    let key = self.to_hashkey(&target_server);
    self.rings.remove_entry(&key);
  }

  fn to_hashkey(&self, new_server: &Node) -> String {
    //TODO: hash
    new_server.url.clone()
  }

  pub fn load_value(&self, key: String) -> Option<String> {
    //TODO: find node for key

    Some(String::from("todo"))
  }
}
