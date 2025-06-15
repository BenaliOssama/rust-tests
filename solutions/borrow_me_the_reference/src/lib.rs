
pub fn delete_and_backspace(s: &mut String) {
    let cp = s.clone();
    s.clear();

    let mut skip = 0 ; 

    for char in cp.chars() {
        if char == '-' {
            s.pop();
        }else if char == '+'{
           skip += 1;
        }else{
            if skip != 0  {
                skip -= 1;
                continue;
            }
            s.push(char);
        }
    }
}

pub fn do_operations(v: &mut [String]) {
    for line in v {
        if line.contains('+') {
            let (num1, num2) = line.split_once('+').unwrap();
            let first: i32 = num1.parse().unwrap();
            let last: i32 = num2.parse().unwrap();
            *line = (first + last).to_string();
        }else if line.contains('-') {
            let (num1, num2) = line.split_once('-').unwrap();
            let first: i32 = num1.parse().unwrap();
            let last: i32 = num2.parse().unwrap();
            *line = (first - last).to_string();
        }
    }
}
