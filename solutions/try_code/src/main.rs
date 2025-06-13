fn main(){
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    let r3 = &mut s;
    let r4 = &mut s;
    let r5 = &mut s;
    println!("{}", r5);
}
