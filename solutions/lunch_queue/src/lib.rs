
#[derive(Debug)]
pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Box<Person>>;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub discount: i32,
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
        let mut first = self.node.clone().unwrap(); 
        let mut second = self.node.clone().unwrap().next_person; 
        first.next_person = None;
        self.node = invert_helper(Some(first), second);
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        if self.node.is_none(){
            return None;
        }

        let mut res : Option::<(String, i32)> = None;
        let mut new_queue = Self::new();

        let mut current = self.node.clone().unwrap();
        while current.next_person.is_some(){
            new_queue.add(current.name, current.discount);
            current = current.next_person.unwrap();
        }
        res = Some((current.clone().name, current.clone().discount));
        new_queue.invert_queue();
        self.node = new_queue.node.clone();
        return res;
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

// fn rm_helper (first:  Option<Box<Person>>,second : Option<Box<Person>>) -> (Option<Box<Person>>,Option<(String, i32)>) {
//     if second.clone().unwrap().next_person.is_none(){
//         let res = (second.clone().unwrap().name.to_string(), second.clone().unwrap().discount);
//         let mut new_first = first.unwrap();//.next_person;
//         new_first.next_person = None;
//         return (Some(new_first), Some(res));
//     }
//     return rm_helper(second.clone(), second.clone().unwrap().next_person);
// }


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
