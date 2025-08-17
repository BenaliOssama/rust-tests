#[derive(Clone, Debug, PartialEq)]
pub struct Table {
	pub headers: Vec<String>,
	pub body: Vec<Vec<String>>,
}

impl Table {
	pub fn new() -> Table {
            return Table{headers: vec![], body:vec![]};
	}

	pub fn add_row(&mut self, row: &[String]) {
            self.body.push(row.to_vec());
	}

	 pub fn filter_col<T:  Fn(&str) -> bool>(&self, filter: T) -> Option<Self> {
            let mut new_table = Table::new();

            let mut column_index = 0; 
            let mut column_name = String::new();
            let mut found = false;

            for (i, col) in self.headers.iter().enumerate() {

                    if filter(&col){
                        column_index = i;
                        column_name= self.headers[i].clone();
                        found = true;
                    }

            }
            if !found{
                return None;
            }
            //let mut names: Vec<String> = vec![];
            for (j, col) in self.body.iter().enumerate() {
               for (i, name)  in col.iter().enumerate(){
                   if i == column_index {
                       let temp = vec![name.to_string()];
                     new_table.add_row(&[temp[0].clone()]);
                   }
               }
            }
            new_table.headers = vec![column_name];
            //new_table.add_row(&names[..]);
            return Some(new_table);
	 }

	// pub fn filter_row<T: Fn(&str) -> bool>(&self, col_name: &str, filter: T) -> Option<Self> {
	//            let mut new_table = Table::new();
	//            for (i, col) in self.body.iter().enumerate() {
	//                let mut all_good = true;
	//
	//                for cell in col.iter(){ 
	//                    if !filter(&col[i]){
	//                        all_good = false;
	//                    }
	//                }
	//
	//                if all_good{ 
	//                     new_table.add_row(col);
	//                }
	//            }
	//            return Some(new_table);
	// }
}
