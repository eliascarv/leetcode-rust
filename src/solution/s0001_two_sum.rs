use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<u32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as u32, j as u32];
            }
        }
    }
    vec![]
}

pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<u32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        match map.get(&diff) {
            None => { 
                map.insert(num, i as u32); 
            }
            Some(val) => {
                return vec![*val, i as u32];
            }
        }
    }
    vec![]
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
    fn test_v1() {
        let nums = vec![2, 7, 11, 15]; 
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_v2() {
        let nums = vec![2, 7, 11, 15]; 
        let target = 9;
        let result = two_sum2(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_v3() {
        let nums = vec![2, 7, 11, 15]; 
        let target = 9;
        let result = two_sum3(nums, target);
        assert_eq!(result, Some((0, 1)));
    }
}
