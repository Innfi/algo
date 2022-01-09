#[cfg(test)]
mod tests {
    use consistent_hash::{Node, StaticHashRing, DefaultHash};
    use sha1::{Sha1, Digest};
    use hex_literal::hex;

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
        hasher.input(b"hello world");

        let result = hasher.result();

        assert_eq!(result[..], hex!("2233"));
    }

    #[test]
    fn test_baseline() {
        let input: i32 = 1;

        assert_eq!(input << 1, 2);
        assert_eq!(input << 2, 4);

        let input1: i32 = 21;

        assert_eq!(input1 << 2, 84);
    }
}
