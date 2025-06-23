fn main(){
    let mut x : u32 = 22 ;
    let y : &u32 = &x;
    x += 1;
    println!("{}",y);
}
