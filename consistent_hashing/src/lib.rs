mod innfis_hash;

#[test]
fn test_instiatiate() {
    let instance = innfis_hash::ConsistentHash::new(vec![]);

    assert_eq!(instance.rings.is_empty(), true);
}

#[test]
fn test_constructor() {
    let node1 = innfis_hash::Node {
        url: String::from("test_url1"),
    };
    let node2 = innfis_hash::Node {
        url: String::from("test_url2"),
    };
    let input_vec = vec![node1, node2];
    let input_len = input_vec.len();

    let instance = innfis_hash::ConsistentHash::new(input_vec);
    assert_eq!(input_len, instance.rings.len());
}

#[test]
fn test_add_node() {
    let input = create_node_input();

    let mut instance = innfis_hash::ConsistentHash::new(input);
    assert_eq!(instance.rings.is_empty(), false);

    let new_node = innfis_hash::Node {
        url: String::from("test8"),
    };

    let add_result = instance.add_node(new_node);
    assert_eq!(add_result, Ok(0));
}

fn create_node_input() -> Vec<innfis_hash::Node> {
    let input_vec = vec![
        innfis_hash::Node {
            url: String::from("test1"),
        },
        innfis_hash::Node {
            url: String::from("test5"),
        },
        innfis_hash::Node {
            url: String::from("test10"),
        },
        innfis_hash::Node {
            url: String::from("test3"),
        },
        innfis_hash::Node {
            url: String::from("test7"),
        },
    ];

    input_vec
}

#[cfg(test)]
mod tests {    
    use consistent_hash::{Node, StaticHashRing, DefaultHash};
    use sha1::{Sha1, Digest};
    //use hex_literal::hex;

    #[test]
    fn test_rust_crate() {
        let nodes = vec![
            Node::new("foo").quantity(5),
            Node::new("bar").quantity(5),
            Node::new("baz").quantity(5),
        ];

        let ring = StaticHashRing::new(DefaultHash, nodes.into_iter());
        assert_eq!(ring.len(), 15);
        assert_eq!(ring.nodes().len(), 3);
        assert_eq!(ring.calc_candidates(&"aa").map(|x| &x.key).collect::<Vec<_>>(),
            [&"bar", &"baz", &"foo"]);
        assert_eq!(ring.calc_candidates(&"bb").map(|x| &x.key).collect::<Vec<_>>(),
            [&"foo", &"bar", &"baz"]);    
    }

    #[test]
    fn test_sha1() {
        let mut hasher = Sha1::new();
        let test_input = b"hello world";
        hasher.input(test_input);

        let result = hasher.result();
        
        //assert_eq!(result[..], hex!("2233"));
    }

    #[test]
    fn test_baseline() {
        let input: i32 = 1;

        assert_eq!(input << 1, 2);
        assert_eq!(input << 2, 4);

        let input1: i32 = 21;

        assert_eq!(input1 << 2, 84);
    }

    #[test]
    fn test_traits() {
        trait Speaks {
            fn speak(&self);
            fn noise(&self) -> &str;
        }

        trait Animal {
            fn animal_type(&self) -> &str;
        }

        struct Dog {}

        impl Animal for Dog {
            fn animal_type(&self) -> &str {
                "dog"
            }
        }

        impl Speaks for Dog {
            fn speak(&self) {
                println!("dog::speaks");
            }

            fn noise(&self) -> &str {
                "bark"
            }
        }

        let a_dog = Dog {};
        assert_eq!(a_dog.animal_type(), "dog");
        assert_eq!(a_dog.noise(), "bark");
    }

    #[test]
    fn try_apply_trait() {
        trait NodeType {
            fn to_type(&self) -> String;
        }

        struct NodeValue {
            node_type: String,
        }

        impl NodeValue {
            pub fn new() -> Self {
                Self { node_type: String::from("hashKey"), }
            }
        }

        impl NodeType for NodeValue {
            fn to_type(&self) -> String {
                self.node_type.clone()
            }
        }

        let node_instance = NodeValue::new();
        assert_eq!(node_instance.to_type(), String::from("hashKey"));
    }

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
}
