use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub struct TinyNode {
  pub url: String,
  pub hash: String,
}

pub struct TinyHashRing {
  rings: Vec<TinyNode>,
  virtual_node_len: usize,
}

fn to_initial_nodes(
  nodes: &Vec<TinyNode>,
  virtual_node_len: usize
) -> Vec<TinyNode> {
  let mut hasher = Sha1::new();

  let mut initial_nodes: Vec<TinyNode> = vec![];

  nodes.into_iter().for_each(|x| {

    //FIXME
    for n in 0..virtual_node_len {
      let node_id: String = to_virtual_node_id(x, n);
      
      hasher.input_str(node_id.as_str());

      let virtual_node = TinyNode {
        url: x.url.clone(),
        hash: hasher.result_str(),
      };

      hasher.reset();

      initial_nodes.push(virtual_node);
    }
  });

  initial_nodes
}

fn to_virtual_node_id(node: &TinyNode, index: usize) -> String {
  if index == 0 { return format!("{}", node.url); }

  format!("{}#{}", node.url, index)
}

impl TinyHashRing {
  // new
  pub fn new(nodes: &Vec<TinyNode>, virtual_node_len: usize) -> Self {
    let mut initial_nodes = to_initial_nodes(nodes, virtual_node_len);
    initial_nodes.sort_by(|a, b| a.hash.cmp(&b.hash));

    Self { rings: initial_nodes, virtual_node_len }
  }

  // rings_view
  pub fn rings_view(&self) -> &Vec<TinyNode> {
    &self.rings
  }

  // add_node
  pub fn add_node(&mut self, new_node: &TinyNode) {
    let mut hasher = Sha1::new();

    let mut new_nodes: Vec<TinyNode> = Vec::new();

    for n in 0..self.virtual_node_len {
      let node_id: String = to_virtual_node_id(new_node, n);
      hasher.input_str(node_id.as_str());
     
      let virtual_node = TinyNode {
        url: new_node.url.clone(),
        hash: hasher.result_str(),
      };

      hasher.reset();

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
    let mut hasher = Sha1::new();

    for n in 0..self.virtual_node_len {
      let node_id: String = to_virtual_node_id(target_node, n);
      hasher.input_str(node_id.as_str());
      let target_hash: String = hasher.result_str();

      let search_result = self.rings.binary_search_by(|x| {
        x.hash.cmp(&target_hash)
      });
      let index: usize = search_result.unwrap_or_else(|x| x);
      self.rings.remove(index);

      hasher.reset();
    }
  }
}
