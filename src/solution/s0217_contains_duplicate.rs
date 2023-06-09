// Problem 217. Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/

use std::collections::HashSet;

pub fn contains_duplicate1(nums: &[i32]) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(num) {
            return true;
        } else {
            set.insert(num);
        }
    }
    false
}

pub fn contains_duplicate2(nums: &[i32]) -> bool {
    let set = HashSet::<_>::from_iter(nums);
    nums.len() > set.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_duplicate_v1() {
        let a = vec![1, 2, 3, 1];
        let b = vec![1, 2, 3, 4];
        let c = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

        assert_eq!(contains_duplicate1(&a), true);
        assert_eq!(contains_duplicate1(&b), false);
        assert_eq!(contains_duplicate1(&c), true);
    }

    #[test]
    fn test_contains_duplicate_v2() {
        let a = vec![1, 2, 3, 1];
        let b = vec![1, 2, 3, 4];
        let c = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

        assert_eq!(contains_duplicate2(&a), true);
        assert_eq!(contains_duplicate2(&b), false);
        assert_eq!(contains_duplicate2(&c), true);
    }
}
