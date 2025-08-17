
#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue{node: None}
    }
    pub fn add(&mut self, name: String, discount: i32) {
        let mut new = Person{
            discount,
            name,
            next_person: None,
        };
        
        if self.node.is_none() {
            self.node = Some(Box::new(new)); 
        }else{
            new.next_person = self.node.clone();
            self.node = Some(Box::new(new));
        }
    }
    pub fn invert_queue(&mut self) {
        if self.node.is_none() {
            return 
        }
        self.node = invert_helper(self.node.clone(), self.node.clone().unwrap().next_person )
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        // 
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut first_person = self.node.clone();
        while first_person.is_some() {
            let person = first_person.clone().unwrap();
            if person.name == name {
                return Some((person.name.to_string(), person.discount));
            }
            first_person = first_person.unwrap().next_person;
        }
        return None;
    }

}



fn invert_helper (first:  Option<Box<Person>>,second : Option<Box<Person>>) -> Option<Box<Person>> {
    if second.is_none(){
        return first;
    }
    let save = second.clone().unwrap().next_person;
    // optioin -> 
    let mut new_second = second.unwrap();//.next_person;// = first;
    new_second.next_person = first; 

    return invert_helper(Some(new_second), save.clone());
}

