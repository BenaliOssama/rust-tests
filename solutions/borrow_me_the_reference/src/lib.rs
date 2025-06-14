
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
}
