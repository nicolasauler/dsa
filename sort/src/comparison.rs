#![allow(dead_code)]
use std::fmt::Debug;

fn bubble_sort<T: Clone + Debug + Ord>(vec: &mut [T]) {
    let len = vec.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

fn select_sort<T: Clone + Debug + Ord>(vec: &mut [T]) {
    let len = vec.len();
    for i in 0..len {
        let mut min_ix = i;
        for j in i..(len - 1) {
            if vec[j + 1] < vec[min_ix] {
                min_ix = j + 1;
            }
        }
        vec.swap(i, min_ix);
    }
}

fn rust_select_sort<T: Clone + Debug + Ord>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let min_ix = vec[i..].iter().enumerate().min_by_key(|v| v.1).unwrap().0;
        // min_ix returns the index relative to the slice
        vec.swap(i, min_ix + i);
    }
}

fn insert_sort<T: Clone + Debug + Ord>(vec: &mut [T]) {
    for i in 1..vec.len() {
        let curr = vec[i].clone();
        for j in (0..i).rev() {
            if curr < vec[j] {
                vec[j + 1] = vec[j].clone();
            } else {
                vec[j + 1] = curr;
                break;
            }
        }
    }
}

fn rec_merge_sort<T: Clone + Debug + Ord>(vec: &Vec<T>) -> Vec<T> {
    let len = vec.len();
    if len < 2 {
        return vec.to_owned();
    } else {
        let half_len = len / 2;
        let left = &rec_merge_sort(&vec[..half_len].to_owned());
        let right = &rec_merge_sort(&vec[half_len..].to_owned());
        return merge(left, right);
    }
}

fn ite_merge_sort<T: Clone + Debug + Ord>(vec: &mut Vec<T>) {
    let len = vec.len();
    let mut size_blk = 2;

    while size_blk < len {
        let mut left_bound = 0;
        while left_bound + size_blk < len {
            let right_bound = (left_bound + 2 * size_blk).min(len);
            let (left, right) = vec[left_bound..right_bound].split_at(size_blk);
            vec.splice(left_bound..right_bound, merge(left, right));
            left_bound += 2 * size_blk;
        }
        size_blk *= 2;
    }
}

fn merge<T: Clone + Debug + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let left_len = left.len();
    let right_len = right.len();

    while i < left_len && j < right_len {
        let left_e = left[i].clone();
        let right_e = right[j].clone();

        if left_e < right_e {
            result.push(left_e);
            i += 1;
        } else {
            result.push(right_e);
            j += 1;
        }
    }

    while i < left_len {
        result.push(left[i].clone());
        i += 1;
    }
    while j < right_len {
        result.push(right[j].clone());
        j += 1;
    }

    return result;
}

fn quick_sort<T: Clone + Debug + Ord>(vec: &mut [T]) {
    let len = vec.len();
    if len < 2 {
        return;
    }

    let partitioned = partition(vec);

    let (left, right) = vec.split_at_mut(partitioned);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn tailless_quick_sort<T: Clone + Debug + Ord>(vec: &mut [T]) {
    let len = vec.len();
    let mut low = 0;
    if len < 2 {
        return;
    }

    while low < len {
        let partitioned = partition(&mut vec[low..]);

        if partitioned == 0 {
            break;
        }
        tailless_quick_sort(&mut vec[low..partitioned]);

        low = partitioned + 1;
    }
}

fn partition<T: Clone + Debug + Ord>(vec: &mut [T]) -> usize {
    let mut smaller = 0;
    let len = vec.len();
    let pivot_ix = len - 1;

    for i in 0..pivot_ix {
        if vec[i] < vec[pivot_ix] {
            vec.swap(i, smaller);
            smaller += 1;
        }
    }
    vec.swap(len - 1, smaller);
    return smaller;
}

fn is_heap<T: Clone + Debug + Ord>(vec: &[T]) -> bool {
    for i in (1..vec.len()).rev() {
        if vec[(i - 1) / 2] < vec[i] {
            return false;
        }
    }
    return true;
}

// from root, build heap iteratively
// at t, vec[0..t] is a heap.
// This is n * log(n) time complexity
fn build_heap<T: Clone + Debug + Ord>(vec: &mut [T]) {
    for i in 1..vec.len() {
        let mut child_ix = i;
        while child_ix > 0 {
            let root_ix = (child_ix - 1) / 2;
            if vec[root_ix] < vec[child_ix] {
                vec.swap(root_ix, child_ix);
            }
            child_ix = root_ix;
        }
    }
}

// Fixes a "almost heap" heap
// In worst case, this is a O(log(n)) operation
fn sift_down<T: Clone + Debug + Ord>(vec: &mut [T], root_ix: usize) {
    let last_ix = vec.len() - 1;
    let mut root_ix = root_ix;
    let mut child_ix = (root_ix + 1) * 2 - 1;

    while child_ix <= last_ix {
        if child_ix < last_ix && vec[child_ix] < vec[child_ix + 1] {
            child_ix += 1;
        }

        if vec[root_ix] < vec[child_ix] {
            vec.swap(root_ix, child_ix);
        } else {
            return;
        }

        root_ix = child_ix;
        child_ix = ((child_ix + 1) * 2) - 1;
    }
}

// Reqiores only O(n) time complexity
fn build_heap_by_sift_down<T: Clone + Debug + Ord>(vec: &mut [T]) {
    let first_parent = (vec.len() / 2) - 1;

    for i in (0..=first_parent).rev() {
        sift_down(vec, i);
    }
}

fn heap_sort<T: Clone + Debug + Ord>(vec: &mut [T]) {
    build_heap_by_sift_down(vec); //now is a heap

    for i in (1..vec.len()).rev() {
        vec.swap(0, i);

        sift_down(&mut vec[..i], 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_ite_merge_sort() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        ite_merge_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_rec_merge_sort() {
        let vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        let new_vec = rec_merge_sort(&vec);
        assert_eq!(new_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_merge() {
        let left = vec![1, 3, 5, 7];
        let right = vec![2, 4, 6, 8];
        let new_vec = merge(&left, &right);
        assert_eq!(new_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_select_sort() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        select_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_rust_select_sort() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        rust_select_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_insert_sort() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        insert_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_quick_sort() {
        let mut vec = vec![10, 80, 30, 90, 40];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![10, 30, 40, 80, 90]);
    }

    #[test]
    fn test_tailless_quick_sort() {
        let mut vec = vec![10, 80, 30, 90, 40];
        tailless_quick_sort(&mut vec);
        assert_eq!(vec, vec![10, 30, 40, 80, 90]);
    }

    #[test]
    fn test_partition() {
        let mut vec = vec![10, 80, 30, 90, 40];
        let pivot_ix = partition(&mut vec);
        assert_eq!(vec, vec![10, 30, 40, 90, 80]);
        assert_eq!(pivot_ix, 2);
    }

    #[test]
    fn test_is_heap() {
        let vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        let is_hea = is_heap(&vec);
        assert_eq!(is_hea, false);
        let vec = vec![8, 7, 6, 5, 4, 3, 2, 1];
        let is_hea = is_heap(&vec);
        assert_eq!(is_hea, true);
        let vec = vec![161, 41, 101, 141, 71, 91, 31, 21, 81, 17, 16];
        let is_hea = is_heap(&vec);
        assert_eq!(is_hea, false);
    }

    #[test]
    fn test_build_heap() {
        let mut vec = vec![161, 41, 101, 141, 71, 91, 31, 21, 81, 17, 16];
        build_heap(&mut vec);
        let is_hea = is_heap(&vec);
        assert_eq!(is_hea, true);
    }

    #[test]
    fn test_build_heap_by_sift_down() {
        let mut vec = vec![161, 41, 101, 141, 71, 91, 31, 21, 81, 17, 16];
        build_heap_by_sift_down(&mut vec);
        let is_hea = is_heap(&vec);
        assert_eq!(is_hea, true);
    }

    #[test]
    fn test_heap_sort() {
        let mut vec = vec![161, 41, 101, 141, 71, 91, 31, 21, 81, 17, 16];
        heap_sort(&mut vec);
        assert_eq!(vec, vec![16, 17, 21, 31, 41, 71, 81, 91, 101, 141, 161]);
    }

    #[test]
    fn test_sift_down() {
        let mut vec = vec![161, 41, 101, 141, 71, 91, 31, 21, 81, 17, 16];
        build_heap_by_sift_down(&mut vec);
        sift_down(&mut vec, 0);
        let is_hea = is_heap(&vec);
        assert_eq!(is_hea, true);
    }
}
