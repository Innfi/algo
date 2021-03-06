use crypto::digest::Digest;
use crypto::sha1::Sha1;

mod innfis_hash;
use innfis_hash::TinyNode;
use innfis_hash::TinyHashRing;
use innfis_hash::HashFunctionTrait;

mod new_hash;
use new_hash::VirtualNode;
use new_hash::HashNodeTrait;

struct HashFunctionSHA1 { }
impl HashFunctionTrait for HashFunctionSHA1 {
  fn to_hash(&self, key: &str) -> String {
    let mut hasher = Sha1::new();

    hasher.input_str(key);
    let hash_value = hasher.result_str();
    hasher.reset();

    hash_value
  }
}

#[test]
fn node_has_basic_fields() {
  let node = TinyNode {
      url: String::from("test_url"),
      hash: String::from(""),
  };

  assert_eq!(node.url.to_string(), "test_url");
}

#[test]
fn access_consistent_hash_with_impl() {
  let node = TinyNode {
    url: String::from("test_url"),
    hash: String::from(""),
  };

  let ring_instance = TinyHashRing::new(&vec![node], 1, HashFunctionSHA1{});

  let view = ring_instance.rings_view();
  assert_eq!(view.len(), 1);
}

#[test]
fn input_nodes_get_hashes() {
  let node = TinyNode {
    url: String::from("test_url"),
    hash: String::from(""),
  };

  let ring_instance = TinyHashRing::new(&vec![node], 1, HashFunctionSHA1{});

  let view = ring_instance.rings_view();
  assert_ne!(view[0].hash, String::from(""));
}

#[test]
fn consistent_hash_has_virtual_node_len() {
  let initial_nodes: Vec<TinyNode> = vec![
    TinyNode {
      url: String::from("http://test-server1:9200"),
      hash: String::from(""),
    },
    TinyNode {
      url: String::from("http://test-server2:9200"),
      hash: String::from(""),
    },
  ];
  let virtual_node_len: usize = 5;
  let expected_node_len: usize = 10;
  
  let ring_instance = TinyHashRing::new(&initial_nodes,
    virtual_node_len,
    HashFunctionSHA1{}
  );

  let view = ring_instance.rings_view();
  assert_eq!(view.len(), expected_node_len);
}

#[test]
fn node_is_sorted_by_hash() {
  let initial_nodes: Vec<TinyNode> = vec![
    TinyNode {
      url: String::from("http://test-server1:9200"),
      hash: String::from(""),
    },
    TinyNode {
      url: String::from("http://test-server2:9200"),
      hash: String::from(""),
    },
  ];
  let virtual_node_len: usize = 3;

  let ring_instance = TinyHashRing::new(&initial_nodes,
    virtual_node_len,
    HashFunctionSHA1{}
  );

  let mut current_hash: String = String::new();

  let view = ring_instance.rings_view();
  view.into_iter().for_each(|x| {
    if !current_hash.is_empty() {
      assert_eq!(x.hash > current_hash, true);
    }

    current_hash = x.hash.clone();
  });
}

#[test]
fn add_node() {
  let initial_nodes: Vec<TinyNode> = vec![
    TinyNode {
      url: String::from("http://test-server1:9200"),
      hash: String::from(""),
    },
    TinyNode {
      url: String::from("http://test-server2:9200"),
      hash: String::from(""),
    },
  ];
  let virtual_node_len: usize = 3;
  let expected_node_len: usize = 9;

  let mut ring_instance = TinyHashRing::new(
    &initial_nodes,
    virtual_node_len,
    HashFunctionSHA1{}
  );

  let new_node = TinyNode {
    url: String::from("http://test-server3:9200"),
    hash: String::from(""),
  };

  ring_instance.add_node(&new_node);

  let view = ring_instance.rings_view();
  assert_eq!(view.len(), expected_node_len);
}

#[test]
fn remove_node() {
  let initial_nodes: Vec<TinyNode> = vec![
    TinyNode {
      url: String::from("http://test-server1:9200"),
      hash: String::from(""),
    },
    TinyNode {
      url: String::from("http://test-server2:9200"),
      hash: String::from(""),
    },
    TinyNode {
      url: String::from("http://test-server3:9200"),
      hash: String::from(""),
    },
  ];
  let virtual_node_len: usize = 3;

  let mut ring_instance = TinyHashRing::new(&initial_nodes,
    virtual_node_len,
    HashFunctionSHA1{}
  );

  let target_node = TinyNode {
    url: String::from("http://test-server2:9200"),
    hash: String::from(""),
  };

  ring_instance.remove_node(&target_node);

  let expected_node_len: usize = 6;
  let view = ring_instance.rings_view();
  assert_eq!(view.len(), expected_node_len);
}

#[test]
fn to_candidate() {
  let initial_nodes: Vec<TinyNode> = vec![
    TinyNode {
      url: String::from("http://test-server1:9200"),
      hash: String::from(""),
    },
    TinyNode {
      url: String::from("http://test-server2:9200"),
      hash: String::from(""),
    },
    TinyNode {
      url: String::from("http://test-server3:9200"),
      hash: String::from(""),
    },
  ];
  let virtual_node_len: usize = 3;

  let mut ring_instance = TinyHashRing::new(
    &initial_nodes,
    virtual_node_len,
    HashFunctionSHA1{}
  );

  let input = String::from("hello, world");

  let candidates = ring_instance.to_candidates(&input);
  candidates.into_iter().for_each(|x| {
    println!("candidate: {}", x);
  });

  ring_instance.rings_view().into_iter().for_each(|x: &TinyNode| {
    println!("url: {}, hash: {}", x.url, x.hash);
  });
}

#[test]
fn test_virtual_node() {
  struct TestNode {
    pub key: String,
  }
  impl HashNodeTrait for TestNode {
    fn to_payload_key(&self) -> &String {
      &self.key
    }
  }

  let payload_instance = TestNode { key: String::from("hello") };
  let node_instance = VirtualNode::new(payload_instance);

  let expected_payload = String::from("hello");

  assert_eq!(node_instance.payload.to_payload_key(), &expected_payload);
}

#[cfg(test)]
mod tests {    
  use crypto::digest::Digest;
  use crypto::sha1::Sha1;

  #[test]
  fn vec_binary_search() {
    let mut preset = vec![ 5, 4, 7, 1, 10, 99, 51 ];
    preset.sort();

    let input = 41;
    let result = preset.binary_search(&input);
    assert_eq!(result, Err(5));

    let out_index = result.unwrap_or_else(|x| x);
    assert_eq!(out_index, 5);

    preset.insert(out_index, input);
    assert_eq!(preset, [1, 4, 5, 7, 10, 41, 51, 99]);
  }

  #[test]
  fn test_crypto_sha1() {
    let mut hasher = Sha1::new();

    hasher.input_str("hello world");
    let result = hasher.result_str();

    assert_eq!(result, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");

    hasher.reset();
    hasher.input_str("hello world1");
    let result1 = hasher.result_str();

    assert_eq!(result1, "c25325615b8492da77c2280a425a3aa82efda6d3");
  }

  fn ring_travel(buffer: &Vec<u32>, index: usize) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();

    for i in index..buffer.len() {
      output.push(buffer[i]);
    }

    for i in 0..index {
      output.push(buffer[i]);
    }

    output
  }

  #[test]
  fn test_ring_buffer() {
    let buffer = vec![ 2, 4, 1, 3, 5, 10, 9 ];
    let index = 3;

    let expected_output = vec![3, 5, 10, 9, 2, 4, 1];
    
    //ring_travel(&buffer, index);
    assert_eq!(ring_travel(&buffer, index), expected_output);
  }

  #[test]
  fn test_generic_traits() {
    trait HashNodeTrait {
      fn to_payload(&self) -> String;
    }

    struct InitialNode {
      url: String,
    }

    impl HashNodeTrait for InitialNode {
      fn to_payload(&self) -> String{
        self.url.clone()
      }
    }

    let instance = InitialNode {
      url: String::from("Hello"),
    };

    assert_eq!(instance.to_payload(), String::from("Hello"));
  }

  #[test]
  fn test_composition_generic() {
    trait HashNodeTrait {
      fn to_payload(&self) -> &String;
    }

    struct ComposedNode<T> where T: HashNodeTrait {
      pub value: T,
      pub hash: String, 
    }

    struct TestHashNode {
      pub url: String,
    }
    impl HashNodeTrait for TestHashNode {
      fn to_payload(&self) -> &String {
        &self.url
      }
    }

    let instance = TestHashNode {
      url: String::from("hello")
    };
    let expected_result: String = String::from("hello");

    assert_eq!(instance.to_payload(), &expected_result);
  }
}
