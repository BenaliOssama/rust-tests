use std::fmt::Formatter;

pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        //return Matrix(vec![vec![]]);
        return Matrix(slice.iter().map(|v| v.to_vec()).collect());
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        let hieght = self.0.len();
        let width = if hieght == 0 {
            0
        }else{
            self.0[0].len()
        };

        let mut result = String::new();

        for j in 0..hieght {
            let mut line = String::from("(");
            for i in 0..width {
                line.push_str(&(self.0[j][i].to_string() + " "));
                //write!(f, "{:?}", self.0[i])?;
            }
            let mut line = String::from(line.trim());
            line.push_str(")\n");
            result.push_str(&line);
        }
        write!(f, "{}", result.trim())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
        let display = String::from("(1 2 3)\n(4 5 6)\n(7 8 9)");
        assert_eq!(display, matrix.to_string());
    }

    #[test]
    fn test_matrix_col() {
        let matrix = Matrix::new(&[&[1], &[2], &[3]]);
        let display = String::from("(1)\n(2)\n(3)");
        assert_eq!(matrix.to_string(), display);
    }
}
