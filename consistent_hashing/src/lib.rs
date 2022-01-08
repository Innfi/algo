#[cfg(test)]
mod tests {
    use consistent_hash::{Node, StaticHashRing, DefaultHash};

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
}
