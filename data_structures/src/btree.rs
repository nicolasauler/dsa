#[derive(Debug)]
struct BTree {
    value: i32,
    left_child: Option<Box<BTree>>,
    right_child: Option<Box<BTree>>,
}

impl BTree {
    fn insert(&mut self, value: i32) {
        let new_sub_tree = Some(Box::new(BTree {
            value,
            left_child: None,
            right_child: None,
        }));

        let mut curr_tree = if value > self.value {
            &mut self.right_child
        } else {
            &mut self.left_child
        };
        while let Some(c_tree) = curr_tree {
            if value > c_tree.value {
                curr_tree = &mut c_tree.right_child;
            } else {
                curr_tree = &mut c_tree.left_child;
            }
        }

        *curr_tree = new_sub_tree;
    }
}

