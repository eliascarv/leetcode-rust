// Problem 150. Evaluate Reverse Polish Notation
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

pub fn eval_rpn1(tokens: Vec<&str>) -> i32 {
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
                _ => unreachable!(),
            };
        }
    }

    filo.pop().unwrap()
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn from_str(token: &str) -> Option<Op> {
        match token {
            "+" => Some(Op::Add),
            "-" => Some(Op::Sub),
            "*" => Some(Op::Mul),
            "/" => Some(Op::Div),
            _ => None,
        }
    }

    fn apply(self, left: i32, right: i32) -> i32 {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        }
    }
}

pub fn eval_rpn2(tokens: Vec<&str>) -> Option<i32> {
    let mut filo = Vec::new();

    for token in tokens {
        if let Some(op) = Op::from_str(token) {
            let right = filo.pop()?;
            let left = filo.pop()?;
            filo.push(op.apply(left, right))
        } else {
            let num = token.parse().expect("integer number");
            filo.push(num);
        }
    }

    filo.pop()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval_rpn1() {
        let tokens: Vec<&str> = vec!["2", "1", "+", "3", "*"];
        assert_eq!(eval_rpn1(tokens), 9);

        let tokens: Vec<&str> = vec!["4", "13", "5", "/", "+"];
        assert_eq!(eval_rpn1(tokens), 6);

        let tokens: Vec<&str> = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        assert_eq!(eval_rpn1(tokens), 22);
    }

    #[test]
    fn test_eval_rpn2() {
        let tokens = vec!["2", "1", "+", "3", "*"];
        assert_eq!(eval_rpn2(tokens), Some(9));

        let tokens = vec!["4", "13", "5", "/", "+"];
        assert_eq!(eval_rpn2(tokens), Some(6));

        let tokens = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        assert_eq!(eval_rpn2(tokens), Some(22));
    }
}
