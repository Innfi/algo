use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub struct TinyNode {
  pub url: String,
  pub hash: String,
}

pub struct TinyHashRing {
  pub rings: Vec<TinyNode>,
}

fn to_initial_nodes(
  nodes: &Vec<TinyNode>,
  virtual_node_len: usize
) -> Vec<TinyNode> {
  let initial_nodes: Vec<TinyNode> = vec![];

  nodes.into_iter().for_each(|x| {

  });

  initial_nodes
}

fn to_virtual_node_id(node: TinyNode, index: usize) -> String {
  format!("{}#{}", node.url, index)
}

impl TinyHashRing {
  pub fn new(nodes: &Vec<TinyNode>, virtual_node_len: usize) -> Self {
    //let mut hasher = Sha1::new();

    // initial_nodes.into_iter().for_each(|mut x| {
    //   hasher.input_str(x.url.as_str());
    //   x.hash = hasher.result_str();
    //   hasher.reset();
    // });

    let initial_nodes = to_initial_nodes(nodes, virtual_node_len);

    Self { rings: initial_nodes }
  }
}