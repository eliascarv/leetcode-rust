// Problem 739. Daily Temperatures
// https://leetcode.com/problems/contains-duplicate/

pub fn daily_temperatures(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut result = vec![0; n];

    for (i, &val) in arr.iter().enumerate() {
        let mut counter = 0;
        for n in (i + 1)..n {
            counter += 1;
            if arr[n] > val {
                result[i] = counter;
                counter = 0;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        let temp = [73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(daily_temperatures(&temp), vec![1, 1, 4, 2, 1, 1, 0, 0]);

        let temp = [30, 40, 50, 60];
        assert_eq!(daily_temperatures(&temp), vec![1, 1, 1, 0]);

        let temp = [30, 60, 90];
        assert_eq!(daily_temperatures(&temp), vec![1, 1, 0]);
    }
}
