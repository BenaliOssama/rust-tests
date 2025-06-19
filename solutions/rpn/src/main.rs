use std::env;

fn rpn(input: &str) {
    let mut stack: Vec<i64> = Vec::new();

    for token in input.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" | "%" => {
                if stack.len() < 2 {
                    println!("Error");
                    return;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match token {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "*" => stack.push(a * b),
                    "/" => {
                        if b == 0 {
                            println!("Error");
                            return;
                        }
                        stack.push(a / b);
                    }
                    "%" => {
                        if b == 0 {
                            println!("Error");
                            return;
                        }
                        stack.push(a % b);
                    }
                    _ => unreachable!(),
                }
            }
            _ => match token.parse::<i64>() {
                Ok(num) => stack.push(num),
                Err(_) => {
                    println!("Error");
                    return;
                }
            },
        }
    }

    if stack.len() != 1 {
        println!("Error");
    } else {
        println!("{}", stack[0]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error");
        return;
    }
    rpn(&args[1]);
}
