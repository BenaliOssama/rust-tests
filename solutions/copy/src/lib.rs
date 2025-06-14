pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).abs().ln())
}


pub fn str_function(a: String) -> (String, String) {
    let splited: Vec<i32> = a.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
    let result: Vec<f64> = splited.into_iter().map(|x| nbr_function(x).1 as f64).collect();
    let mut res = String::new();

    for (i, r) in result.iter().enumerate() {
        res.push_str(&r.to_string());
        if i != result.len() - 1 {
            res.push_str(" ");
        }
    }

    (a, res)
}



pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {

    let result: Vec<f64> = b.clone().into_iter().map(|x| nbr_function(x).2 as f64).collect();
    let mut res : Vec<f64> = [].to_vec() ;

    for  r in result{
        res.push(r);
    }

    (b, res)
}

