extern crate crypto;
extern crate hex;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::iter::repeat;
use std::fmt;

use crate::binary::Node;

type MerkleNode = Node<Vec<u8>>;

impl MerkleNode {
    fn new(data: Vec<u8>) -> MerkleNode {
        Node{left: None, right: None, data: data}
    }

    fn from(left: Option<Box<MerkleNode>>, right: Option<Box<MerkleNode>>) -> MerkleNode {
        let mut dig = Sha256::new();
        let mut sha: Vec<u8> = repeat(0).take(32).collect();

        match &left {
            Some(node) => dig.input(&node.data),
            None => {},
        };

        match &right {
            Some(node) => dig.input(&node.data),
            None => {},
        };

        dig.result(&mut sha);
        Node{left: left, right: right, data: sha}
    }

}

impl fmt::Display for MerkleNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.data.as_slice()))
    }
}

fn build(prev: &Vec<MerkleNode>) -> MerkleNode {
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

    build(&next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_children() {
        let root = MerkleNode::from(
            Some(Box::new(MerkleNode::new(String::from("hello").into_bytes()))),
            Some(Box::new(MerkleNode::new(String::from("world").into_bytes()))),
        );
        assert_eq!(hex::encode(root.data.as_slice()), "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af")
    }

    #[test]
    fn build_tree() {
        let leaves = vec![
            MerkleNode::new(String::from("hello").into_bytes()),
            MerkleNode::new(String::from("world").into_bytes()),
        ];
        
        let root = build(&leaves);
        assert_eq!(hex::encode(root.data.as_slice()), "936a185caaa266bb9cbe981e9e05cb78cd732b0b3280eb944412bb6f8f8f07af")
    }
}