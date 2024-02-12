#![allow(dead_code)]
use std::{collections::HashMap, fmt::Debug};

fn binary_search<T>(arr: &[T], target: T) -> Option<usize>
where
    T: Debug + Ord,
{
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let middle = (left + right) / 2;
        println!("middle: {}", middle);
        println!("arr[middle]: {:?}", arr[middle]);
        match arr[middle].cmp(&target) {
            std::cmp::Ordering::Less => left = middle + 1,
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Greater => {
                if middle <= 0 {
                    return None;
                }
                right = middle - 1;
            }
        }
    }

    return None;
}

fn rec_binary_search<T>(arr: &[T], target: T) -> Option<usize>
where
    T: Debug + Ord,
{
    let len = arr.len();
    let middle = (len - 1) / 2;

    match arr[middle].cmp(&target) {
        std::cmp::Ordering::Less => {
            if len < 2 {
                return None;
            }
            return rec_binary_search(&arr[middle + 1..], target).map(|i| i + middle + 1);
        }
        std::cmp::Ordering::Equal => return Some(middle),
        std::cmp::Ordering::Greater => {
            if len < 2 {
                return None;
            }
            return rec_binary_search(&arr[..middle], target);
        }
    }
}

fn two_sum(nums: &[i32], target: i32) -> Vec<usize> {
    for i in 0..nums.len() {
        let needle = target - nums[i];
        if let Some(pos) = binary_search(&nums[i + 1..], needle) {
            return vec![i, i + pos + 1];
        }
    }

    unreachable!()
}

fn two_sum_leet(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    nums.sort_unstable_by_key(|t| t.1);

    for i in 0..nums.len() {
        let needle = target - nums[i].1;
        if let Ok(pos) = nums[i + 1..].binary_search_by_key(&needle, |&t| *t.1) {
            return vec![nums[i].0 as i32, nums[i + pos + 1].0 as i32];
        }
    }

    unreachable!()
}

fn two_sum_leet_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    let mut hash: HashMap<i32, usize> = HashMap::with_capacity(len);
    for i in 0..len {
        hash.insert(nums[i], i);
    }

    for i in 0..len {
        let needle = target - nums[i];
        if let Some(pos) = hash.get(&needle) {
            if *pos == i {
                continue;
            }
            return vec![i as i32, *pos as i32];
        }
    }
    unreachable!()
}

fn naive_calculate_pow(base: u32, exp: u32) -> u32 {
    let mut result = base;

    for _ in 0..(exp - 1) {
        result *= base;
    }

    return result;
}

fn calculate_pow(base: u32, exp: u32) -> u32 {
    if exp == 0 {
        return 1;
    } else if exp % 2 == 0 {
        return calculate_pow(base * base, exp / 2);
    } else {
        return base * calculate_pow(base * base, (exp - 1) / 2);
    }
}

fn main() {
    let base = 2;
    let exp = 4;
    let result = calculate_pow(base, exp);
    println!("result: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_search_some() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for i in 0..arr.len() {
            let target = arr[i];
            let result = binary_search(&arr, target);
            assert_eq!(result, Some(i));
        }
    }

    #[test]
    fn test_binary_search_none() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let needle = 0;
        let result = binary_search(&arr, needle);
        assert_eq!(result, None)
    }

    #[test]
    fn test_binary_search_rec_some() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for i in 0..arr.len() {
            let target = arr[i];
            let result = rec_binary_search(&arr, target);
            assert_eq!(result, Some(i));
        }
    }

    #[test]
    fn test_binary_search_rec_none() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let needle = 0;
        let result = rec_binary_search(&arr, needle);
        assert_eq!(result, None)
    }

    #[test]
    fn test_two_sum_1() {
        let nums = [2, 7, 11, 15];
        let target = 9;
        let result = two_sum(&nums, target);
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn test_two_sum_2() {
        let nums = [3, 2, 4];
        let target = 6;
        let result = two_sum(&nums, target);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn test_two_sum_3() {
        let nums = [3, 3];
        let target = 6;
        let result = two_sum(&nums, target);
        assert_eq!(result, [0, 1]);
    }
    #[test]
    fn test_two_sum_1_leet() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum_leet(nums, target);
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn test_two_sum_2_leet() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum_leet(nums, target);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn test_two_sum_3_leet() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum_leet(nums, target);
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn test_two_sum_4_leet() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let result = two_sum_leet(nums, target);
        assert_eq!(result, [0, 3]);
    }

    #[test]
    fn test_two_sum_1_leet_hash() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum_leet_hash(nums, target);
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn test_two_sum_2_leet_hash() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum_leet_hash(nums, target);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn test_two_sum_3_leet_hash() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum_leet_hash(nums, target);
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn test_two_sum_4_leet_hash() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let result = two_sum_leet_hash(nums, target);
        assert_eq!(result, [0, 3]);
    }

    #[test]
    fn test_dumb_calculate_pow() {
        let base = 2;
        let exp = 4;
        let result = naive_calculate_pow(base, exp);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_dumb_calculate_pow_2() {
        let base = 3;
        let exp = 3;
        let result = naive_calculate_pow(base, exp);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_calculate_pow() {
        let base = 2;
        let exp = 4;
        let result = calculate_pow(base, exp);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_calculate_pow_2() {
        let base = 3;
        let exp = 3;
        let result = calculate_pow(base, exp);
        assert_eq!(result, 27);
    }
}
