// Problem 136. Single Number
// https://leetcode.com/problems/single-number/

pub fn single_number1(nums: &[i32]) -> i32 {
    let mut seen = Vec::new();
    for &num in nums {
        if seen.contains(&num) {
            seen.pop();
        } else {
            seen.push(num);
        }
    }
    seen[0]
}

pub fn single_number2(nums: &[i32]) -> i32 {
    let mut single = 0;
    for num in nums {
        single ^= num;
    }
    single
}

pub fn single_number3(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, x| acc ^ x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number_v1() {
        let a = vec![2, 2, 1];
        let b = vec![4, 1, 2, 1, 2];
        let c = vec![1];
        assert_eq!(single_number1(&a), 1);
        assert_eq!(single_number1(&b), 4);
        assert_eq!(single_number1(&c), 1);
    }

    #[test]
    fn test_single_number_v2() {
        let a = vec![2, 2, 1];
        let b = vec![4, 1, 2, 1, 2];
        let c = vec![1];
        assert_eq!(single_number2(&a), 1);
        assert_eq!(single_number2(&b), 4);
        assert_eq!(single_number2(&c), 1);
    }

    #[test]
    fn test_single_number_v3() {
        let a = vec![2, 2, 1];
        let b = vec![4, 1, 2, 1, 2];
        let c = vec![1];
        assert_eq!(single_number3(&a), 1);
        assert_eq!(single_number3(&b), 4);
        assert_eq!(single_number3(&c), 1);
    }
}
