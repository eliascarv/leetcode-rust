use std::collections::HashMap;

pub fn two_sum1(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let diff = target - num;
        match map.get(&diff) {
            Some(&value) => {
                return Some((value, i));
            }
            None => {
                map.insert(num, i);
            }
        }
    }
    None
}

pub fn two_sum3(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&value) = map.get(&diff) {
            return Some((value, i));
        } else {
            map.insert(num, i);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum_v1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum1(nums, target);
        assert_eq!(result, Some((0, 1)));

        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum1(nums, target);
        assert_eq!(result, Some((1, 2)));

        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum1(nums, target);
        assert_eq!(result, Some((0, 1)));

        let nums = vec![3, 2, 1];
        let target = 6;
        let result = two_sum1(nums, target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_two_sum_v2() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum2(nums, target);
        assert_eq!(result, Some((0, 1)));

        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum2(nums, target);
        assert_eq!(result, Some((1, 2)));

        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum2(nums, target);
        assert_eq!(result, Some((0, 1)));

        let nums = vec![3, 2, 1];
        let target = 6;
        let result = two_sum2(nums, target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_two_sum_v3() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum3(nums, target);
        assert_eq!(result, Some((0, 1)));

        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum3(nums, target);
        assert_eq!(result, Some((1, 2)));

        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum3(nums, target);
        assert_eq!(result, Some((0, 1)));

        let nums = vec![3, 2, 1];
        let target = 6;
        let result = two_sum3(nums, target);
        assert_eq!(result, None);
    }
}
