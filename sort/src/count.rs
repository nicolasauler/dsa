#![allow(dead_code)]
use std::{collections::BTreeMap, fmt::Debug};

fn counting_sort<T: Clone + Debug + Into<usize> + From<usize>>(vec: &mut [T]) {
    let len = vec.len();
    let mut counter: Vec<usize> = Vec::with_capacity(len);

    for counter_ix in vec.iter() {
        let counter_ix: usize = counter_ix.clone().into();
        match counter.get_mut(counter_ix) {
            Some(v) => *v += 1,
            None => {
                counter.resize(counter_ix + 1, 0);
                counter[counter_ix] += 1;
            }
        }
    }

    let mut vec_ix = 0;
    for (i, &count) in counter.iter().enumerate() {
        for _ in 0..count {
            vec[vec_ix] = i.into();
            vec_ix += 1;
        }

        if vec_ix == len {
            break;
        }
        if count > 1 {
            vec_ix += count - 1;
        }
    }

    println!("counter: {:?}", counter);
}

fn counting_sort_with_max<T>(vec: &mut [T])
where
    T: Clone + Debug + Ord + Into<usize> + From<usize>,
{
    let max: usize = vec.iter().max().unwrap().clone().into();
    let mut counter: Vec<usize> = vec![0; max + 1];

    for counter_ix in vec.iter() {
        let counter_ix: usize = counter_ix.clone().into();
        counter[counter_ix] += 1;
    }

    let mut vec_ix = 0;
    for (i, &count) in counter.iter().enumerate() {
        for _ in 0..count {
            vec[vec_ix] = i.into();
            vec_ix += 1;
        }
    }

    println!("counter: {:?}", counter);
}

fn counting_sort_hash<T: Clone + Debug + Ord>(vec: &mut [T]) {
    let mut counter = BTreeMap::new();

    for counter_ix in vec.iter() {
        let counter_ix = counter_ix.clone();
        match counter.get_mut(&counter_ix) {
            Some(v) => *v += 1,
            None => {
                counter.insert(counter_ix, 1);
            }
        }
    }

    let mut vec_ix = 0;
    for (i, &count) in counter.iter() {
        for _ in 0..count {
            vec[vec_ix] = i.clone();
            vec_ix += 1;
        }
    }

    println!("counter: {:?}", counter);
}

fn bucket_sort(vec: &mut [u32]) {
    let len = vec.len();
    let mut counter: Vec<Vec<u32>> = Vec::with_capacity(10);
    for _ in 0..10 {
        counter.push(Vec::new());
    }

    let mut max_magnitude = 0;
    for elem in vec.iter() {
        let mut magnitude = 1;
        while *elem >= 10_u32.pow(magnitude) {
            magnitude += 1;
        }
        if magnitude > max_magnitude {
            max_magnitude = magnitude;
        }
    }

    for k in 0..max_magnitude {
        for elem in vec.iter() {
            //let counter_ix = (elem % 10_u32.pow(k + 1)) / 10_u32.pow(k);
            let counter_ix = (elem / 10_u32.pow(k)) % 10;
            counter[counter_ix as usize].push(*elem);
        }

        let mut vec_ix = 0;
        for bucket in counter.iter_mut() {
            for _ in 0..bucket.len() {
                vec[vec_ix] = bucket.remove(0);
                vec_ix += 1;
            }

            if vec_ix == len {
                break;
            }
            if bucket.len() > 1 {
                vec_ix += bucket.len() - 1;
            }
        }
    }
}

fn get_max_mag(vec: &mut [u32]) -> u32 {
    let mut max_mag: u32 = 0;

    for &elem in vec.iter() {
        let mut mag: u32 = 0;
        while elem >= 10_u32.pow(mag) {
            mag += 1;
        }

        if mag > max_mag {
            max_mag = mag;
        }
    }

    return max_mag;
}

fn get_prefix_sum(vec: &mut [u32], alg: u32) -> [usize; 10] {
    let mut count: [usize; 10] = [0; 10];

    // compute algarism counts
    for elem in vec.iter() {
        let mag_al = (*elem / 10_u32.pow(alg)) % 10;
        count[mag_al as usize] += 1;
    }

    // compute prefix_sum
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    return count;
}

fn radix_sort(vec: &mut [u32]) {
    let mut output: Vec<u32> = vec![0; vec.len()];
    let num_algs = get_max_mag(vec) + 1;

    for alg in 0..num_algs {
        let mut prefix_sum = get_prefix_sum(vec, alg);

        for &elem in vec.iter().rev() {
            let mag_alg = (elem / 10_u32.pow(alg)) % 10;

            let position = &mut prefix_sum[mag_alg as usize];
            *position -= 1;

            output[*position] = elem;
        }

        vec.copy_from_slice(&output);
    }
}

pub fn test_count_sorts() {
    let mut vec = vec![1, 3, 2, 5, 4, 7];
    counting_sort(&mut vec);
    println!("new_vec: {vec:?}");

    let mut vec = vec![1, 3, 2, 5, 4, 7];
    counting_sort_hash(&mut vec);
    println!("new_vec: {vec:?}");

    let mut vec = vec![1, 3, 2, 5, 4, 7];
    counting_sort_with_max(&mut vec);
    println!("new_vec: {vec:?}");

    let mut vec: Vec<u32> = vec![10, 30, 20, 50, 40, 60, 13, 15, 49, 17, 70, 48, 80];
    bucket_sort(&mut vec);
    println!("new_vec: {vec:?}");

    let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
    radix_sort(&mut vec);
    println!("new_vec: {vec:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_counting_sort() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        counting_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_counting_sort_hash() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        counting_sort_hash(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_counting_sort_with_max() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        counting_sort_with_max(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_bucket_sort_1_mag() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        bucket_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_bucket_sort_2_mag() {
        let mut vec = vec![10, 30, 20, 50, 40, 60, 13, 15, 49, 17, 70, 48, 80];
        bucket_sort(&mut vec);
        assert_eq!(
            vec,
            vec![10, 13, 15, 17, 20, 30, 40, 48, 49, 50, 60, 70, 80]
        );
    }

    #[test]
    fn test_radix_sort_1_mag() {
        let mut vec = vec![1, 3, 2, 5, 4, 6, 7, 8];
        radix_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_radix_sort_2_mag() {
        let mut vec = vec![10, 30, 20, 50, 40, 60, 13, 15, 49, 17, 70, 48, 80];
        radix_sort(&mut vec);
        assert_eq!(
            vec,
            vec![10, 13, 15, 17, 20, 30, 40, 48, 49, 50, 60, 70, 80]
        );
    }

    #[test]
    fn test_radix_sort_mixed_mag() {
        let mut vec = vec![
            10, 30, 20, 50, 40, 60, 13, 15, 49, 17, 70, 48, 80, 100, 200, 500, 400, 600, 130, 150,
            490, 170, 700, 480, 800,
        ];
        radix_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                10, 13, 15, 17, 20, 30, 40, 48, 49, 50, 60, 70, 80, 100, 130, 150, 170, 200, 400,
                480, 490, 500, 600, 700, 800
            ]
        );
    }
}
