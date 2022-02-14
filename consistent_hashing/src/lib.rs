mod innfis_hash;

#[test]
fn node_has_basic_fields() {
  let node = innfis_hash::TinyNode {
      url: String::from("test_url"),
      hash: String::from(""),
  };

  assert_eq!(node.url.to_string(), "test_url");
}

#[test]
fn consistent_hash_has_empty_ring() {
  let ring_instance = innfis_hash::TinyHashRing {
    rings: vec![]
  };

  assert_eq!(ring_instance.rings.is_empty(), true);
}

#[test]
fn access_consistent_hash_with_impl() {
  let node = innfis_hash::TinyNode {
    url: String::from("test_url"),
    hash: String::from(""),
  };

  let ring_instance = innfis_hash::TinyHashRing::new(&vec![node], 1);

  assert_eq!(ring_instance.rings.len(), 1);
}

#[test]
fn input_nodes_get_hashes() {
  let node = innfis_hash::TinyNode {
    url: String::from("test_url"),
    hash: String::from(""),
  };

  let ring_instance = innfis_hash::TinyHashRing::new(&vec![node], 1);

  assert_ne!(ring_instance.rings[0].hash, String::from(""));
}

#[test]
fn consistent_hash_has_virtual_node_len() {
  let node = innfis_hash::TinyNode {
    url: String::from("test_url"),
    hash: String::from(""),
  };

  let virtual_node_len: usize = 5;
  let ring_instance = innfis_hash::TinyHashRing::new(
    &vec![node],
    virtual_node_len
  );

  assert_eq!(ring_instance.rings.len(), virtual_node_len);
  ring_instance.rings.into_iter().for_each(|x| {
    println!("url: {}, hash: {}", x.url, x.hash);
  });
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
}
