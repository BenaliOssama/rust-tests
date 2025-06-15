#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self{
            alias: alias.to_string(),
            brightness: 0 
        }
    }
    pub fn change(&mut self, new_value: u8) {
        self.brightness = new_value;
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for l in lights {
        if l.alias == alias{
            l.change(value);
        }
    }
}
