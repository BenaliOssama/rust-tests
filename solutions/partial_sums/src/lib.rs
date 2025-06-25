pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut res : Vec<u64> = [].to_vec();
    for i in (0..arr.len()).rev()  {
        let mut sum = 0 ; 
        for j in 0..=i {
            sum += arr[j];
        }
        res.push(sum);
    }
    res.push(0);
    res
}
