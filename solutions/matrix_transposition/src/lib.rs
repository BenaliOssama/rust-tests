#[derive(Debug)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));


pub fn transpose(m: Matrix) -> Matrix {
    // `let` can be used to bind the members of a tuple to variables.
    let Matrix((a, b), (c,d))  = m;
    Matrix((a,c),(b,d))
}
