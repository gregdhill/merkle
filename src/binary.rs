#[derive(Debug, Default, Clone)]
pub struct Node<T> {
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub data: T,
}

impl Node<i64> {
    pub fn new(value: i64) -> Node<i64> {
        Node{left: None, right: None, data: value}
    }

    pub fn insert(&mut self, value: i64) {
        if value < self.data {
            match &self.left {
                Some(_node) => {
                    let mut left = self.left.take().unwrap();
                    left.insert(value);
                    self.left = Some(left);
                    return
                },
                None => {
                    self.left = Some(Box::new(Node{ left: None, right: None, data: value }));
                    return
                },
            }
        } else {
            match &self.right {
                Some(_node) => {
                    let mut right = self.right.take().unwrap();
                    right.insert(value);
                    self.right = Some(right);
                },
                None => {
                    self.right = Some(Box::new(Node { left: None, right: None, data: value }));
                },
            }
        }
    }

    pub fn lookup(&self, target: i64) -> bool {
        if target == self.data {
            return true;
        }
        
        if target < self.data {
            return match &self.left {
                Some(node) => node.lookup(target),
                None => false,
            };
        }

        return match &self.right {
            Some(node) => node.lookup(target),
            None => false,
        };
    }

    pub fn dfs(&self, callback: &mut FnMut(i64)) {
        callback(self.data);
        match &self.left {
            Some(node) => node.dfs(callback),
            None => (),
        };
        match &self.right {
            Some(node) => node.dfs(callback),
            None => (),
        };
    }

    fn max(&self) -> i64 {
        return match &self.right {
            Some(node) => node.max(),
            None => self.data, 
        };
    }

    fn min(&self) -> i64 {
        return match &self.left {
            Some(node) => node.min(),
            None => self.data, 
        };
    }

    // TODO: size
    // TODO: maxDepth
    // TODO: minValue
    // TODO: printTree
    // TODO: sameTree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_right() {
        let mut root = Node::new(0);
        root.insert(1);
        assert_eq!(root.right.unwrap().data, 1);
    }

    #[test]
    fn insert_left() {
        let mut root = Node::new(1);
        root.insert(0);
        assert_eq!(root.left.unwrap().data, 0);
    }

    #[test]
    fn count_nodes() {
        let root = Node{
            left: Some(Box::new(Node::new(0))),
            right: Some(Box::new(Node::new(2))),
            data: 1,
        };

        let mut count = 0;
        root.dfs(&mut |_| { count+=1; });
        assert_eq!(count, 3);
    }

    #[test]
    fn min_value() {
        let mut root = Node::new(10);
        root.insert(9);
        root.insert(8);
        root.insert(7);
        root.insert(6);
        root.insert(5);

        let min = root.min();
        assert_eq!(min, 5);

        let mut root = Node::new(0);
        root.insert(1);

        let min = root.min();
        assert_eq!(min, 0);
    }
}