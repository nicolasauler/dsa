#[derive(Debug)]
struct AVLNode<T: Ord> {
    value: T,
    height: usize,
    balance_factor: i8,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLNode<T> {
    fn new(value: T) -> Self {
        AVLNode {
            value,
            height: 0,
            balance_factor: 0,
            left: None,
            right: None,
        }
    }

    fn update(&mut self) {
        let maybe_left_height: &Option<usize> = &self.left.as_ref().map(|node| node.height);
        let maybe_right_height: &Option<usize> = &self.right.as_ref().map(|node| node.height);

        // compute max of left and right inside the option
        let height = maybe_left_height
            .max(maybe_right_height)
            .map(|x| x + 1)
            .unwrap_or(0);

        self.height = height;
    }
}

#[derive(Debug)]
struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    len: usize,
}

impl<T: Ord> AVLTree<T> {
    fn insert(&mut self, value: T) -> bool {
        fn _insert<T: Ord>(node: &mut Option<Box<AVLNode<T>>>, value: T) -> bool {
            let mut node: &mut Option<Box<AVLNode<T>>> = if let Some(root) = node.as_mut() {
                if value > root.value {
                    &mut root.right
                } else {
                    &mut root.left
                }
            } else {
                *node = Some(Box::new(AVLNode::new(value)));
                return true;
            };

            while let Some(next_node) = node {
                if value > next_node.value {
                    node = &mut next_node.left;
                } else {
                    node = &mut next_node.right;
                }
            }

            *node = Some(Box::new(AVLNode::new(value)));
            return true;
        }

        let is_success = _insert(&mut self.root, value);
        if is_success {
            self.len += 1;
        }
        return is_success;
    }
}

fn test() {
    let mut avl_tree: AVLTree<i32> = AVLTree { root: None, len: 0 };

    avl_tree.insert(10);
    println!("{:#?}", avl_tree);
    avl_tree.insert(20);
    println!("{:#?}", avl_tree);
    avl_tree.insert(5);
    println!("{:#?}", avl_tree);
}
