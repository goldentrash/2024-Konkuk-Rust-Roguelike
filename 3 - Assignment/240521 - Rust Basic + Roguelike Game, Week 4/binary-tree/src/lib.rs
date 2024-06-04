// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

// A container storing a set of values, using a binary tree.
//
// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.
impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self {
            root: Subtree(None),
        }
    }

    fn insert(&mut self, trg: T) {
        let mut subtree = &mut self.root;

        while subtree.0.is_some() {
            let node = subtree.0.as_deref_mut().unwrap();
            let val = &node.value;

            if trg < *val {
                subtree = &mut node.left;
            } else if trg > *val {
                subtree = &mut node.right;
            } else {
                return;
            }
        }

        subtree.0 = Some(Box::new(Node {
            value: trg,
            left: Subtree(None),
            right: Subtree(None),
        }));
    }

    fn len(&self) -> u32 {
        let mut q = Vec::<&Subtree<T>>::new();
        let mut ret = 0;

        q.push(&self.root);
        while let Some(curr) = q.pop() {
            if curr.0.is_none() {
                continue;
            }

            ret += 1;

            let node = curr.0.as_ref().unwrap();
            q.push(&node.left);
            q.push(&node.right);
        }

        ret
    }

    fn has(&self, trg: &T) -> bool {
        let mut subtree = &self.root;

        while subtree.0.is_some() {
            let node = subtree.0.as_ref().unwrap();
            let val = &node.value;

            if *trg < *val {
                subtree = &node.left;
            } else if *trg > *val {
                subtree = &node.right;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();

        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();

        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|val| tree.has(&(val as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();

        for i in 0..100 {
            tree.insert(i);
        }

        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
