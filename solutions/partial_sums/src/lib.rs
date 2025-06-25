pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut res : Vec<u64> = [].to_vec();
    for i in (0..arr.len()).rev()  {
        res.push(arr[0..=i].iter().copied().reduce(|a, b| a + b).unwrap());
    }
    res.push(0);
    res
}
