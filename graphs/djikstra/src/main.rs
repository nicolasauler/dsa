#![allow(dead_code)]

use std::fmt::Debug;
mod lazy;

#[derive(Debug, PartialEq, Eq)]
struct AdjacencyElement {
    to_node: usize,
    weight: i32,
}

impl PartialOrd for AdjacencyElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AdjacencyElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

#[derive(Debug)]
/// Example:    values  [3, 15, 11, 17, 7, 9, 2, 1, 6, 5, 16, 4]
///             pm      [2, 9, 11, 7, 8, 6, 1, 0, 3, 4, 10, 5]
///             im      [7, 6, 0, 8, 9, 11, 5, 3, 4, 1, 10, 2]
/// In this structure, we know that key index 3, has a value of 17, and is in the binary heap array
/// node index 7
/// Also, we know that for heap node index 7, the associated key index is 3
struct IndexedPriorityQueue<T: Clone + Debug + Ord> {
    /// the values to be ordered/stored in the heap, indexed by key
    values: Vec<T>,
    /// Index of the node in the heap for a given key index
    pm: Vec<usize>,
    /// Inverse map: maps key index for a given heap node index
    im: Vec<usize>,
}

impl<T: Clone + Debug + Ord> IndexedPriorityQueue<T> {
    fn swap(&mut self, child_node: usize, parent_node: usize) {
        let child_key = self.im[child_node];
        let parent_key = self.im[parent_node];
        self.pm[parent_key] = child_node;
        self.pm[child_key] = parent_node;
        self.im[child_node] = parent_key;
        self.im[parent_node] = child_key;
    }

    fn swim(&mut self, key_index: usize) {
        let mut node_index = self.pm[key_index];
        while let Some(parent) = node_index.checked_sub(1).map(|x| x / 2) {
            let parent_key_index = self.im[parent];
            if self.values[parent_key_index] > self.values[key_index] {
                self.swap(node_index, parent);
                node_index = parent;
            } else {
                break;
            }
        }
    }

    fn push(&mut self, new_value: T) -> usize {
        let last_pos = self.im.len();

        self.values.push(new_value);
        self.pm.push(last_pos);
        self.im.push(last_pos);

        self.swim(last_pos);

        return last_pos;
    }

    /// also called sift_dow
    fn sink(&mut self, node_index: usize) {
        let mut curr_node_index = node_index;
        let mut left_child_node_ix = (node_index * 2) + 1;
        if left_child_node_ix >= self.im.len() {
            return;
        }
        let mut left_child_key_ix = self.im[left_child_node_ix];

        while let Some(left_child) = self.values.get(left_child_key_ix) {
            let right_child_key_ix = self
                .im
                .get(left_child_node_ix + 1)
                .expect("create logic for non existing right child");
            let maybe_right_child = self.values.get(*right_child_key_ix);
            let child_key_ix = match maybe_right_child {
                Some(right_child) => {
                    if right_child < left_child {
                        *right_child_key_ix
                    } else {
                        left_child_key_ix
                    }
                }
                None => left_child_key_ix,
            };

            let curr_key_ix = self.im[curr_node_index];
            let curr_value = &self.values[curr_key_ix];

            if *curr_value > self.values[child_key_ix] {
                self.swap(node_index, self.pm[child_key_ix]);
                curr_node_index = self.pm[child_key_ix];
                left_child_node_ix = (curr_node_index * 2) + 1;
                left_child_key_ix = self.im[left_child_node_ix];
            } else {
                break;
            }
        }
    }

    fn erase_last_value(&mut self, node_index: usize) -> Option<T> {
        let key_index = self.im[node_index];
        let old_value = self.values.get(key_index).cloned();
        self.im.pop();
        self.pm[key_index] = usize::MAX;

        return old_value;
    }

    fn pop(&mut self) -> Option<T> {
        let last_node_ix = self.im.len() - 1;
        if last_node_ix <= 0 {
            return None;
        }

        self.swap(0, last_node_ix);
        let erased_value = self.erase_last_value(last_node_ix);
        self.sink(0);

        return erased_value;
    }

    fn remove(&mut self, key_ix: usize) -> Option<T> {
        let node_ix = self.pm[key_ix];
        if node_ix == usize::MAX {
            return None;
        }
        if let None = self.im.get(node_ix) {
            return None;
        }

        let last_node_ix = self.im.len() - 1;
        if last_node_ix <= 0 {
            return None;
        }

        self.swap(node_ix, last_node_ix);
        let erased_value = self.erase_last_value(last_node_ix);
        self.swim(node_ix);
        self.sink(node_ix);

        return erased_value;
    }

    fn update(&mut self, key_ix: usize, new_value: T) -> Option<T> {
        let node_ix = self.pm[key_ix];
        if node_ix == usize::MAX {
            return None;
        }
        if let None = self.im.get(node_ix) {
            return None;
        }

        let old_value = self.values[key_ix].clone();
        self.values[key_ix] = new_value;
        self.swim(node_ix);
        self.sink(node_ix);

        return Some(old_value);
    }
}

fn main() {
    let mut ipq = IndexedPriorityQueue {
        values: vec![3, 15, 11, 17, 7, 9, 2, 1, 6, 5, 16, 4],
        pm: vec![2, 9, 11, 7, 8, 6, 1, 0, 3, 4, 10, 5],
        im: vec![7, 6, 0, 8, 9, 11, 5, 3, 4, 1, 10, 2],
    };
    ipq.push(2);
    let popped = ipq.pop();
    println!("popped: {popped:?}");
    let removed = ipq.remove(11);
    println!("removed: {removed:?}");
    let updated = ipq.update(2, 1);
    println!("updated: {updated:?}");
}
