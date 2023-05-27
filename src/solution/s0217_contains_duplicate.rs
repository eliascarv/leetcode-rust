use std::collections::HashSet;

pub fn contains_duplicate(nums: &[i32]) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(num) {
            return true;
        } else {
            set.insert(num);
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let a = vec![1, 2, 3, 1];
        let b = vec![1, 2, 3, 4];
        let c = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

        assert_eq!(contains_duplicate(&a), true);
        assert_eq!(contains_duplicate(&b), false);
        assert_eq!(contains_duplicate(&c), true);
    }
}
