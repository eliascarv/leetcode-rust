// Problem 150. Evaluate Reverse Polish Notation
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

pub fn eval_rpn(tokens: Vec<&str>) -> i32 {
    let mut filo: Vec<i32> = Vec::new();

    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            filo.push(num);
        } else {
            let right: i32 = filo.pop().unwrap();
            let left: i32 = filo.pop().unwrap();
            match token {
                "+" => filo.push(left + right),
                "-" => filo.push(left - right),
                "*" => filo.push(left * right),
                "/" => filo.push(left / right),
                _   => unreachable!(),
            };
        }
    }

    filo.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        let tokens: Vec<&str> = vec!["2","1","+","3","*"];
        assert_eq!(eval_rpn(tokens), 9);

        let tokens: Vec<&str> = vec!["4","13","5","/","+"];
        assert_eq!(eval_rpn(tokens), 6);

        let tokens: Vec<&str> = vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"];
        assert_eq!(eval_rpn(tokens), 22);
    }
}