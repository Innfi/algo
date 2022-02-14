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
  pub fn new(nodes: &Vec<TinyNode>, virtual_node_len: usize) -> Self {
    let initial_nodes = to_initial_nodes(nodes, virtual_node_len);

    Self { rings: initial_nodes }
  }
}
