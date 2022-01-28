mod innfis_hash;

#[test]
fn test_mod() {
    let instance = innfis_hash::ConsistentHash::new();

    assert_eq!(instance.initial_runner(), String::from("start from here"));
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
    fn try_apply_trait_to_consistent_hash() {
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
}
