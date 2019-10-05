mod tree {
    extern crate crypto;
    extern crate hex;

    use self::crypto::digest::Digest;
    use self::crypto::sha2::Sha256;
    use std::iter::repeat;

    #[derive(Debug, Default)]
    pub struct Merkle {
        pub root: Node,
    }

    impl Merkle {
        fn new(data: Vec<&[u8]>) -> Merkle {
            let mut leaves = vec![Node::default(); data.len()];
            for (i, item) in data.iter().enumerate() {
                leaves[i] = Node{ left: None, right: None, data: item.to_vec() };
            }
            
            Merkle {
                root: prop(&leaves)
            }
        }
    }

    fn prop(prev: &Vec<Node>) -> Node {
        let mut next = vec![Node::default(); ((prev.len() as f64) / 2.0).round() as usize];
        for i in (0..prev.len()).step_by(2) {
            let mut dig = Sha256::new();
            let mut sha: Vec<u8> = repeat(0).take(32).collect();

            match prev.get(i) {
                Some(node) => {
                    dig.input(&*prev[i].data);
                    next[i/2].left = Some(Box::new(node.clone()));
                },
                None => {}
            };

            match prev.get(i+1) {
                Some(node) => {
                    dig.input(&*prev[i+1].data);
                    next[i/2].right = Some(Box::new(node.clone()));
                },
                None => {}
            };
            
            dig.result(&mut sha);
            next[i/2].data = sha;
        }

        if next.len() == 1 {
            return next[0].clone()
        }

        prop(&next)
    }

    #[derive(Debug, Default, Clone)]
    pub struct Node {
        pub left: Option<Box<Node>>,
        pub right: Option<Box<Node>>,
        pub data: Vec<u8>,
    }

    #[test]
    fn root_hash() {
        let tree = Merkle::new(vec!("hello".as_bytes(), "world".as_bytes()));
        assert_eq!(hex::encode(tree.root.data.as_slice()), "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af")
    }
}