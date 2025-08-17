use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
	pub headers: Vec<String>,
	pub body: Vec<Vec<String>>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.body.len() == 0 {
            return Ok(());
        }
        /*_____________header______________________*/
        let mut line = String::from("|");
        for (i, header) in self.headers.iter().enumerate(){
            let max = self.find_max(i) + 2;
            write!(f, "|{:^1$}",header, max );
            line.push_str(&("-".repeat(max).to_string() + "+"));
        }
        write!(f, "|\n");
        let line = line.strip_suffix("+").unwrap();
        writeln!(f, "{}", line.to_string() + "|");
        
        /*_____________body_________________________*/
        for (j, row) in self.body.iter().enumerate() {
            for (i, line) in row.iter().enumerate(){
                let max = self.find_max(i) + 2;
                write!(f, "|{:^1$}",line, max );
            }
            if j != self.body.len() -1 {
                write!(f, "|\n");
            }else{
                write!(f,"|");
            }
        }
        Ok(())
    }
}


impl Table {
    pub fn new() -> Table {
        return Table{headers: vec![], body: vec![]};
    }
    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
    fn find_max(&self, col : usize) -> usize {
        let mut max = 0;
        for j in 0..self.body.len() {
            for (index,word) in self.body[j].iter().enumerate(){
                if max < word.len() && index == col {
                    max = word.len();
                }
            }
        }
        return max;
    }
}
