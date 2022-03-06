use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub struct TinyNode {
  pub url: String,
  pub hash: String,
}

pub trait HashFunctionTrait {
  fn to_hash(&self, key: &str) -> String;
}

pub struct TinyHashRing<F> {
  rings: Vec<TinyNode>,
  virtual_node_len: usize,
  hash_function: F,
}

impl<F> TinyHashRing<F> where F: HashFunctionTrait {
  // new
  pub fn new(
    nodes: &Vec<TinyNode>,
    virtual_node_len: usize,
    hash_function: F
  ) -> Self {
    let mut initial_nodes: Vec<TinyNode> = vec![];
    nodes.into_iter().for_each(|x| {
      for n in 0..virtual_node_len {
        let node_id: String = format!("{}#{}", x.url, n);
        initial_nodes.push(TinyNode {
          url: x.url.clone(),
          hash: hash_function.to_hash(&node_id.to_string()),
        });
      }
    });
    initial_nodes.sort_by(|a, b| a.hash.cmp(&b.hash));

    Self { rings: initial_nodes, virtual_node_len, hash_function }
  }

  // rings_view
  pub fn rings_view(&self) -> &Vec<TinyNode> {
    &self.rings
  }

  // add_node
  pub fn add_node(&mut self, new_node: &TinyNode) {
    let mut new_nodes: Vec<TinyNode> = Vec::new();

    for n in 0..self.virtual_node_len {
      let node_id: String = format!("{}#{}", new_node.url, n);
      let virtual_node = TinyNode {
        url: new_node.url.clone(),
        hash: self.hash_function.to_hash(&node_id),
      };

      new_nodes.push(virtual_node);
    }

    self.add_nodes(new_nodes);
  }

  fn add_nodes(&mut self, new_nodes: Vec<TinyNode>) {
    for new_node in new_nodes {
      let search_result = self.rings.binary_search_by(|x| {
        x.hash.cmp(&new_node.hash)
      });

      let index: usize = search_result.unwrap_or_else(|x| x);

      self.rings.insert(index, new_node);
    }
  }

  // remove_node
  pub fn remove_node(&mut self, target_node: &TinyNode) {
    for n in 0..self.virtual_node_len {
      let node_id: String = format!("{}#{}", target_node.url, n);
      let target_hash: String = self.hash_function.to_hash(&node_id);

      let search_result = self.rings.binary_search_by(|x| {
        x.hash.cmp(&target_hash)
      });
      let index: usize = search_result.unwrap_or_else(|x| x);
      self.rings.remove(index);
    }
  }

  // find candidates 
  pub fn to_candidates(&mut self, key: &String) -> Vec<String> {
    let target_hash: String = self.hash_function.to_hash(&key.as_str());

    let search_result = self.rings.binary_search_by(|x| { 
      x.hash.cmp(&target_hash)
    });
    let index: usize = search_result.unwrap_or_else(|x| x);

    let mut candidates: Vec<String> = Vec::new();
    //FIXME
    for i in index..self.rings.len() {
      candidates.push(self.rings[i].url.clone());
    }
    for i in 0..index {
      candidates.push(self.rings[i].url.clone());
    }

    candidates
  }
}
