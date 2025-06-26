pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    if i == 0 {
        return [].to_vec();
    }
    let mut res : Vec<String> = [].to_vec();
    for j in 1..=i{
        res.push(" ".repeat(j as usize) + &v.repeat(j as usize));// v.clone() + &" ");
    }
    let ex = res.clone();
    res.extend(ex[0..ex.len() - 1].iter().cloned().rev());
    res
}
