use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Blog {
    pub fn new() -> Blog {
        return Blog{drops: Cell::new(0), states: RefCell::new(vec![])};
    }
    pub fn new_article(&self, body: String) -> (usize, Article) {
        let new_id = self.new_id();
        let new_article = Article::new(new_id , body, self);
        self.states.borrow_mut().push(false);
        return (new_id, new_article);
    }
    pub fn new_id(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        return self.states.borrow()[id];
    }
    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            panic!("{} is already dropped", id);
        }
        self.drops.set(self.drops.get() + 1);
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Article<'a> {
    id: usize,
    body: String,
    parent : &'a Blog,
}

impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, blog: &'a Blog) -> Article {
        return Article{id, body, parent: blog};
    }
    pub fn discard(self) {
        self.parent.add_drop(self.id);
        self.parent.states.borrow_mut()[self.id] = true ;//.push(true);
        let _ = self;
    }
}
